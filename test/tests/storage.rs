use compallm_core::persistent::storage::user_space::*;
use compallm_storage::{S3Storage, S3UserSpace};
use std::time::{Duration, SystemTime};

fn constraints() -> AvatarUploadConstraints {
    AvatarUploadConstraints {
        min_bytes: 4,
        max_bytes: 8,
        content_type: "image/png".into(),
        expires_at: SystemTime::now() + Duration::from_secs(300),
    }
}

async fn upload(
    client: &reqwest::Client,
    upload: &AvatarUpload,
    bytes: &[u8],
) -> reqwest::Response {
    let mut form = reqwest::multipart::Form::new();
    for (key, value) in &upload.fields {
        form = form.text(key.clone(), value.clone());
    }
    form = form.part(
        "file",
        reqwest::multipart::Part::bytes(bytes.to_vec()).file_name("avatar.png"),
    );
    client
        .post(&upload.url)
        .header("Origin", "http://localhost:3007")
        .multipart(form)
        .send()
        .await
        .unwrap()
}

#[tokio::test]
#[ignore = "requires scripts/setup-storage.sh; no database is accessed"]
async fn direct_avatar_upload_and_post_enforcement() {
    let config = loco_rs::environment::Environment::Development
        .load_from_folder(std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../config"
        )))
        .unwrap();
    let settings: compallm_storage::S3Settings =
        serde_json::from_value(config.settings.unwrap()["s3"].clone()).unwrap();
    let base = settings.public_base_url.clone();
    let storage: Box<dyn UserStorageSpace> = Box::new(S3UserSpace::new(std::sync::Arc::new(
        S3Storage::new(settings.clone()).unwrap(),
    )));
    let user = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64;
    let client = reqwest::Client::new();
    let cancelled = hero::cancellation::CancellationToken::new();
    cancelled.cancel();
    assert!(matches!(
        cancelled
            .scope(storage.generate_avatar_upload(user, &constraints()))
            .await,
        Err(StorageError::Cancelled)
    ));
    let backend = S3Storage::new(settings.clone()).unwrap();
    let key = S3UserSpace::get_avatar_path(user)
        .trim_start_matches('/')
        .to_owned();
    let public_url = backend.public_url(&key).unwrap();
    assert_eq!(public_url, format!("{base}/avatars/{user}"));
    assert!(backend.public_url("../private").is_err());
    let mut authorization = storage
        .generate_avatar_upload(user, &constraints())
        .await
        .unwrap();
    assert_eq!(authorization.fields["key"], key);
    assert!(
        client
            .get(&public_url)
            .send()
            .await
            .unwrap()
            .status()
            .is_client_error()
    );
    let preflight = client
        .request(reqwest::Method::OPTIONS, &authorization.url)
        .header("Origin", "http://localhost:3007")
        .header("Access-Control-Request-Method", "POST")
        .header("Access-Control-Request-Headers", "content-type")
        .send()
        .await
        .unwrap();
    assert!(preflight.status().is_success());
    assert!(
        preflight
            .headers()
            .contains_key("access-control-allow-origin")
    );
    assert!(
        client
            .put(&public_url)
            .body("denied")
            .send()
            .await
            .unwrap()
            .status()
            .is_client_error()
    );
    assert!(
        client
            .get(&authorization.url)
            .send()
            .await
            .unwrap()
            .status()
            .is_client_error()
    );
    for bytes in [b"1234".as_slice(), b"12345678".as_slice()] {
        let response = upload(&client, &authorization, bytes).await;
        assert!(response.status().is_success());
        assert!(
            response
                .headers()
                .contains_key("access-control-allow-origin")
        );
        let delivered = client
            .get(&public_url)
            .header("Origin", "http://localhost:3007")
            .send()
            .await
            .unwrap();
        assert!(delivered.status().is_success());
        assert!(storage.avatar_uploaded(user).await.unwrap());
        assert_eq!(delivered.headers()["content-type"], "image/png");
        assert!(
            delivered
                .headers()
                .contains_key("access-control-allow-origin")
        );
        assert_eq!(delivered.bytes().await.unwrap().as_ref(), bytes);
    }
    for bytes in [b"123".as_slice(), b"123456789".as_slice()] {
        assert!(
            upload(&client, &authorization, bytes)
                .await
                .status()
                .is_client_error()
        );
    }
    authorization.fields.insert(
        "key".into(),
        S3UserSpace::get_avatar_path(user + 1)
            .trim_start_matches('/')
            .to_owned(),
    );
    assert!(
        upload(&client, &authorization, b"1234")
            .await
            .status()
            .is_client_error()
    );
    authorization.fields.insert("key".into(), key.clone());
    authorization
        .fields
        .insert("Content-Type".into(), "text/plain".into());
    assert!(
        upload(&client, &authorization, b"1234")
            .await
            .status()
            .is_client_error()
    );
    authorization
        .fields
        .insert("Content-Type".into(), "image/png".into());
    authorization.fields.get_mut("policy").unwrap().push('A');
    assert!(
        upload(&client, &authorization, b"1234")
            .await
            .status()
            .is_client_error()
    );
    let mut short = constraints();
    short.expires_at = SystemTime::now() + Duration::from_secs(1);
    let expired = storage.generate_avatar_upload(user, &short).await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert!(
        upload(&client, &expired, b"1234")
            .await
            .status()
            .is_client_error()
    );
    assert_eq!(
        client
            .get(&public_url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap()
            .as_ref(),
        b"12345678"
    );
    use compallm_core::persistent::storage::org_space::OrganizationStorage;
    let org_storage = compallm_storage::OrgStorage::new(std::sync::Arc::new(
        S3Storage::new(settings.clone()).unwrap(),
    ));
    let org = user + 1;
    let mut org_upload = org_storage
        .generate_avatar_upload(org, &constraints())
        .await
        .unwrap();
    assert!(!org_storage.avatar_uploaded(org).await.unwrap());
    assert_eq!(org_upload.fields["key"], format!("avatars/{org}"));
    assert!(
        upload(&client, &org_upload, b"abcd")
            .await
            .status()
            .is_success()
    );
    assert!(org_storage.avatar_uploaded(org).await.unwrap());
    assert_eq!(
        client
            .get(backend.public_url(&org_storage.avatar_path(org)).unwrap())
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap()
            .as_ref(),
        b"abcd"
    );
    assert_eq!(
        client
            .get(&public_url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap()
            .as_ref(),
        b"12345678"
    );
    org_upload.fields.insert("key".into(), key.clone());
    assert!(
        upload(&client, &org_upload, b"abcd")
            .await
            .status()
            .is_client_error()
    );
    let sdk = hero::s3::S3Config {
        region: settings.region,
        access_key_id: settings.access_key,
        secret_access_key: settings.secret_key,
        endpoint_url: Some(settings.endpoint),
        force_path_style: settings.force_path_style,
    }
    .build_client();
    sdk.delete_object(&hero::s3::ObjectPath::new(
        &settings.bucket,
        format!("avatars/{org}"),
    ))
    .await
    .unwrap();
    sdk.delete_object(&hero::s3::ObjectPath::new(settings.bucket, key))
        .await
        .unwrap();
}

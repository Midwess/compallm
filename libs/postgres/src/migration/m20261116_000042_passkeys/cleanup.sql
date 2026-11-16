CREATE FUNCTION public.compallm_remove_passkey_credential() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog, public AS $$
BEGIN
    IF OLD.passkey_id IS NOT NULL THEN
        DELETE FROM public.webauthn_credentials WHERE id = OLD.passkey_id;
    END IF;
    RETURN OLD;
END;
$$;
CREATE TRIGGER user_identity_passkey_cleanup AFTER DELETE ON public.user_identities
FOR EACH ROW EXECUTE FUNCTION public.compallm_remove_passkey_credential();

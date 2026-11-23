const artifact = {
    "name": "SettingsProfileUserProfile",
    "kind": "HoudiniQuery",
    "hash": "5844c279151695f1a7ceadad06b09ccb1259ad6c06fed2dd11de68668275e192",
    "raw": `query SettingsProfileUserProfile($userProfileId: ID!) {
    userProfile(id: $userProfileId) {
        node {
            id
            displayName
            username
            email
            avatarUrl
            __typename
        }
        __typename
    }
}
`,

    "rootType": "Query",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "userProfile": {
                "type": "UserPayload",
                "keyRaw": "userProfile(id: $userProfileId)",
                "nullable": true,

                "selection": {
                    "fields": {
                        "node": {
                            "type": "User",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
                                        "visible": true,
                                    },

                                    "displayName": {
                                        "type": "String",
                                        "keyRaw": "displayName",
                                        "visible": true,
                                    },

                                    "username": {
                                        "type": "String",
                                        "keyRaw": "username",
                                        "visible": true,
                                    },

                                    "email": {
                                        "type": "String",
                                        "keyRaw": "email",
                                        "visible": true,
                                    },

                                    "avatarUrl": {
                                        "type": "String",
                                        "keyRaw": "avatarUrl",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "__typename": {
                                        "type": "String",
                                        "keyRaw": "__typename",
                                    },
                                },
                            },

                            "visible": true,
                        },

                        "__typename": {
                            "type": "String",
                            "keyRaw": "__typename",
                        },
                    },
                },

                "visible": true,
            },
        },
    },

    "pluginData": {},

    "input": {
        "fields": {
            "userProfileId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type SettingsProfileUserProfile = {
	readonly "input": SettingsProfileUserProfile$input;
	readonly "result": SettingsProfileUserProfile$result | undefined;
};

export type SettingsProfileUserProfile$result = {
	/**
	 * Read an active user profile using verified user identity.
	 */
	readonly userProfile: {
		readonly node: {
			/**
			 * Stable user identity, serialized as a string.
			 */
			readonly id: string;
			/**
			 * Display name; this field does not establish identity or uniqueness.
			 */
			readonly displayName: string;
			readonly username: string;
			/**
			 * Email is readable by authenticated users and cannot be updated here.
			 */
			readonly email: string;
			/**
			 * Public avatar URL; the object may not exist yet and clients must show a fallback.
			 */
			readonly avatarUrl: string | null;
		};
	} | null;
};

export type SettingsProfileUserProfile$input = {
	userProfileId: string;
};

export type SettingsProfileUserProfile$unmasked = {
	/**
	 * Read an active user profile using verified user identity.
	 */
	readonly userProfile: {
		readonly node: {
			/**
			 * Stable user identity, serialized as a string.
			 */
			readonly id: string;
			/**
			 * Display name; this field does not establish identity or uniqueness.
			 */
			readonly displayName: string;
			readonly username: string;
			/**
			 * Email is readable by authenticated users and cannot be updated here.
			 */
			readonly email: string;
			/**
			 * Public avatar URL; the object may not exist yet and clients must show a fallback.
			 */
			readonly avatarUrl: string | null;
			readonly __typename: "User";
		};
		readonly __typename: "UserPayload";
	} | null;
};

export type SettingsProfileUserProfile$artifact = typeof artifact

"HoudiniHash=5844c279151695f1a7ceadad06b09ccb1259ad6c06fed2dd11de68668275e192"

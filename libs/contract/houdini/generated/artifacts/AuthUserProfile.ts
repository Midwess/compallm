const artifact = {
    "name": "AuthUserProfile",
    "kind": "HoudiniQuery",
    "hash": "1e90d696b24269cf64178d03aae87710468fa9a5b18309e01fae2e69db8c1279",
    "raw": `query AuthUserProfile($userProfileId: ID!) {
    userProfile(id: $userProfileId) {
        node {
            email
            __typename
            id
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
                                    "email": {
                                        "type": "String",
                                        "keyRaw": "email",
                                        "visible": true,
                                    },

                                    "__typename": {
                                        "type": "String",
                                        "keyRaw": "__typename",
                                    },

                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
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

export type AuthUserProfile = {
	readonly "input": AuthUserProfile$input;
	readonly "result": AuthUserProfile$result | undefined;
};

export type AuthUserProfile$result = {
	/**
	 * Read an active user profile using verified user identity.
	 */
	readonly userProfile: {
		readonly node: {
			/**
			 * Email is readable by authenticated users and cannot be updated here.
			 */
			readonly email: string;
		};
	} | null;
};

export type AuthUserProfile$input = {
	userProfileId: string;
};

export type AuthUserProfile$unmasked = {
	/**
	 * Read an active user profile using verified user identity.
	 */
	readonly userProfile: {
		readonly node: {
			/**
			 * Email is readable by authenticated users and cannot be updated here.
			 */
			readonly email: string;
			readonly __typename: "User";
			/**
			 * Stable user identity, serialized as a string.
			 */
			readonly id: string;
		};
		readonly __typename: "UserPayload";
	} | null;
};

export type AuthUserProfile$artifact = typeof artifact

"HoudiniHash=1e90d696b24269cf64178d03aae87710468fa9a5b18309e01fae2e69db8c1279"

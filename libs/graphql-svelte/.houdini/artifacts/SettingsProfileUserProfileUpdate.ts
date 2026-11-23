import type { UserProfileUpdateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsProfileUserProfileUpdate",
    "kind": "HoudiniMutation",
    "hash": "243ca2bbde21177b40da16b0bd62543da4c2706fb37f12b5a9c683f0fce9d76c",
    "raw": `mutation SettingsProfileUserProfileUpdate($userProfileUpdateInput: UserProfileUpdateInput!) {
    userProfileUpdate(input: $userProfileUpdateInput) {
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

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "userProfileUpdate": {
                "type": "UserPayload",
                "keyRaw": "userProfileUpdate(input: $userProfileUpdateInput)",

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
            "userProfileUpdateInput": "UserProfileUpdateInput",
        },

        "types": {
            "UserProfileUpdateInput": {
                "avatarUploaded": "Boolean",
                "displayName": "String",
                "username": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsProfileUserProfileUpdate = {
	readonly "input": SettingsProfileUserProfileUpdate$input;
	readonly "result": SettingsProfileUserProfileUpdate$result;
};

export type SettingsProfileUserProfileUpdate$result = {
	/**
	 * Update the current user's display name and confirm an uploaded avatar.
	 */
	readonly userProfileUpdate: {
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
	};
};

export type SettingsProfileUserProfileUpdate$input = {
	userProfileUpdateInput: UserProfileUpdateInput;
};

export type SettingsProfileUserProfileUpdate$optimistic = {
	/**
	 * Update the current user's display name and confirm an uploaded avatar.
	 */
	readonly userProfileUpdate?: {
		readonly node?: {
			/**
			 * Stable user identity, serialized as a string.
			 */
			readonly id?: string;
			/**
			 * Display name; this field does not establish identity or uniqueness.
			 */
			readonly displayName?: string;
			readonly username?: string;
			/**
			 * Email is readable by authenticated users and cannot be updated here.
			 */
			readonly email?: string;
			/**
			 * Public avatar URL; the object may not exist yet and clients must show a fallback.
			 */
			readonly avatarUrl?: string | null;
		};
	};
};

export type SettingsProfileUserProfileUpdate$unmasked = {
	/**
	 * Update the current user's display name and confirm an uploaded avatar.
	 */
	readonly userProfileUpdate: {
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
	};
};

export type SettingsProfileUserProfileUpdate$artifact = typeof artifact

"HoudiniHash=243ca2bbde21177b40da16b0bd62543da4c2706fb37f12b5a9c683f0fce9d76c"

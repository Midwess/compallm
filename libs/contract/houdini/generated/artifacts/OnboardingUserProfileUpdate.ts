import type { UserProfileUpdateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "OnboardingUserProfileUpdate",
    "kind": "HoudiniMutation",
    "hash": "30a61bfa9718f93e83e4e88124c1b81583b9c59c09dd9acbd975074cd0a46b28",
    "raw": `mutation OnboardingUserProfileUpdate($userProfileUpdateInput: UserProfileUpdateInput!) {
    userProfileUpdate(input: $userProfileUpdateInput) {
        node {
            displayName
            username
            email
            avatarUrl
            __typename
            id
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

export type OnboardingUserProfileUpdate = {
	readonly "input": OnboardingUserProfileUpdate$input;
	readonly "result": OnboardingUserProfileUpdate$result;
};

export type OnboardingUserProfileUpdate$result = {
	/**
	 * Update the current user's display name and confirm an uploaded avatar.
	 */
	readonly userProfileUpdate: {
		readonly node: {
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

export type OnboardingUserProfileUpdate$input = {
	userProfileUpdateInput: UserProfileUpdateInput;
};

export type OnboardingUserProfileUpdate$optimistic = {
	/**
	 * Update the current user's display name and confirm an uploaded avatar.
	 */
	readonly userProfileUpdate?: {
		readonly node?: {
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

export type OnboardingUserProfileUpdate$unmasked = {
	/**
	 * Update the current user's display name and confirm an uploaded avatar.
	 */
	readonly userProfileUpdate: {
		readonly node: {
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
			/**
			 * Stable user identity, serialized as a string.
			 */
			readonly id: string;
		};
		readonly __typename: "UserPayload";
	};
};

export type OnboardingUserProfileUpdate$artifact = typeof artifact

"HoudiniHash=30a61bfa9718f93e83e4e88124c1b81583b9c59c09dd9acbd975074cd0a46b28"

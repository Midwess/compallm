import type { EmailInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "AuthInvitationControllerEmailSignupRequest",
    "kind": "HoudiniMutation",
    "hash": "6cbe5445a831ad18b807e421bfde27bb0f9a1aea320946b09617f4842f599992",
    "raw": `mutation AuthInvitationControllerEmailSignupRequest($emailSignupRequestInput: EmailInput!) {
    emailSignupRequest(input: $emailSignupRequestInput) {
        node {
            lifetime {
                expiresInSeconds
                __typename
            }
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
            "emailSignupRequest": {
                "type": "EmailChallengePayload",
                "keyRaw": "emailSignupRequest(input: $emailSignupRequestInput)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "EmailChallenge",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "lifetime": {
                                        "type": "TokenLifetime",
                                        "keyRaw": "lifetime",

                                        "selection": {
                                            "fields": {
                                                "expiresInSeconds": {
                                                    "type": "String",
                                                    "keyRaw": "expiresInSeconds",
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
            "emailSignupRequestInput": "EmailInput",
        },

        "types": {
            "EmailInput": {
                "delivery": "CredentialDelivery",
                "email": "String",
                "invitation": "Boolean",
                "purpose": "AuthenticationPurpose",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type AuthInvitationControllerEmailSignupRequest = {
	readonly "input": AuthInvitationControllerEmailSignupRequest$input;
	readonly "result": AuthInvitationControllerEmailSignupRequest$result;
};

export type AuthInvitationControllerEmailSignupRequest$result = {
	/**
	 * Request a signup email challenge; reject existing accounts before sending.
	 */
	readonly emailSignupRequest: {
		readonly node: {
			readonly lifetime: {
				readonly expiresInSeconds: string;
			};
		};
	};
};

export type AuthInvitationControllerEmailSignupRequest$input = {
	emailSignupRequestInput: EmailInput;
};

export type AuthInvitationControllerEmailSignupRequest$optimistic = {
	/**
	 * Request a signup email challenge; reject existing accounts before sending.
	 */
	readonly emailSignupRequest?: {
		readonly node?: {
			readonly lifetime?: {
				readonly expiresInSeconds?: string;
			};
		};
	};
};

export type AuthInvitationControllerEmailSignupRequest$unmasked = {
	/**
	 * Request a signup email challenge; reject existing accounts before sending.
	 */
	readonly emailSignupRequest: {
		readonly node: {
			readonly lifetime: {
				readonly expiresInSeconds: string;
				readonly __typename: "TokenLifetime";
			};
			readonly __typename: "EmailChallenge";
		};
		readonly __typename: "EmailChallengePayload";
	};
};

export type AuthInvitationControllerEmailSignupRequest$artifact = typeof artifact

"HoudiniHash=6cbe5445a831ad18b807e421bfde27bb0f9a1aea320946b09617f4842f599992"

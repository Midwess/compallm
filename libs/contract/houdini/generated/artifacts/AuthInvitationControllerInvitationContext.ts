const artifact = {
    "name": "AuthInvitationControllerInvitationContext",
    "kind": "HoudiniQuery",
    "hash": "25bca59d83f6f984a2b93c65b242f048e4ab3e42dce030f9b07fa06a9ec11e71",
    "raw": `query AuthInvitationControllerInvitationContext {
    invitationContext {
        node {
            organizationName
            email
            expiresAt
            signup
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
            "invitationContext": {
                "type": "InvitationContextPayload",
                "keyRaw": "invitationContext",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "InvitationContext",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "organizationName": {
                                        "type": "String",
                                        "keyRaw": "organizationName",
                                        "visible": true,
                                    },

                                    "email": {
                                        "type": "String",
                                        "keyRaw": "email",
                                        "visible": true,
                                    },

                                    "expiresAt": {
                                        "type": "Float",
                                        "keyRaw": "expiresAt",
                                        "visible": true,
                                    },

                                    "signup": {
                                        "type": "Boolean",
                                        "keyRaw": "signup",
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
    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type AuthInvitationControllerInvitationContext = {
	readonly "input"?: AuthInvitationControllerInvitationContext$input;
	readonly "result": AuthInvitationControllerInvitationContext$result | undefined;
};

export type AuthInvitationControllerInvitationContext$result = {
	readonly invitationContext: {
		readonly node: {
			readonly organizationName: string;
			readonly email: string;
			readonly expiresAt: number;
			readonly signup: boolean;
		};
	};
};

export type AuthInvitationControllerInvitationContext$input = null | undefined;

export type AuthInvitationControllerInvitationContext$unmasked = {
	readonly invitationContext: {
		readonly node: {
			readonly organizationName: string;
			readonly email: string;
			readonly expiresAt: number;
			readonly signup: boolean;
			readonly __typename: "InvitationContext";
		};
		readonly __typename: "InvitationContextPayload";
	};
};

export type AuthInvitationControllerInvitationContext$artifact = typeof artifact

"HoudiniHash=25bca59d83f6f984a2b93c65b242f048e4ab3e42dce030f9b07fa06a9ec11e71"

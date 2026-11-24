import type { InvitationCreateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsMembersInvitationCreate",
    "kind": "HoudiniMutation",
    "hash": "5a1047a8074e75e66a354bcb518e9d2b0cd097d8f4c600efebbba2c5e3c89bca",
    "raw": `mutation SettingsMembersInvitationCreate($invitationCreateInput: InvitationCreateInput!) {
    invitationCreate(input: $invitationCreateInput) {
        node {
            invitation {
                id
                email
                status
                expiresAt
                issuedAt
                __typename
            }
            url
            emailAdmitted
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
            "invitationCreate": {
                "type": "InvitationIssuedPayload",
                "keyRaw": "invitationCreate(input: $invitationCreateInput)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "InvitationIssued",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "invitation": {
                                        "type": "Invitation",
                                        "keyRaw": "invitation",

                                        "selection": {
                                            "fields": {
                                                "id": {
                                                    "type": "ID",
                                                    "keyRaw": "id",
                                                    "visible": true,
                                                },

                                                "email": {
                                                    "type": "String",
                                                    "keyRaw": "email",
                                                    "visible": true,
                                                },

                                                "status": {
                                                    "type": "String",
                                                    "keyRaw": "status",
                                                    "visible": true,
                                                },

                                                "expiresAt": {
                                                    "type": "Float",
                                                    "keyRaw": "expiresAt",
                                                    "visible": true,
                                                },

                                                "issuedAt": {
                                                    "type": "Float",
                                                    "keyRaw": "issuedAt",
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

                                    "url": {
                                        "type": "String",
                                        "keyRaw": "url",
                                        "visible": true,
                                    },

                                    "emailAdmitted": {
                                        "type": "Boolean",
                                        "keyRaw": "emailAdmitted",
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
            "invitationCreateInput": "InvitationCreateInput",
        },

        "types": {
            "InvitationCreateInput": {
                "email": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsMembersInvitationCreate = {
	readonly "input": SettingsMembersInvitationCreate$input;
	readonly "result": SettingsMembersInvitationCreate$result;
};

export type SettingsMembersInvitationCreate$result = {
	readonly invitationCreate: {
		readonly node: {
			readonly invitation: {
				readonly id: string;
				readonly email: string;
				readonly status: string;
				readonly expiresAt: number;
				readonly issuedAt: number;
			};
			readonly url: string;
			readonly emailAdmitted: boolean;
		};
	};
};

export type SettingsMembersInvitationCreate$input = {
	invitationCreateInput: InvitationCreateInput;
};

export type SettingsMembersInvitationCreate$optimistic = {
	readonly invitationCreate?: {
		readonly node?: {
			readonly invitation?: {
				readonly id?: string;
				readonly email?: string;
				readonly status?: string;
				readonly expiresAt?: number;
				readonly issuedAt?: number;
			};
			readonly url?: string;
			readonly emailAdmitted?: boolean;
		};
	};
};

export type SettingsMembersInvitationCreate$unmasked = {
	readonly invitationCreate: {
		readonly node: {
			readonly invitation: {
				readonly id: string;
				readonly email: string;
				readonly status: string;
				readonly expiresAt: number;
				readonly issuedAt: number;
				readonly __typename: "Invitation";
			};
			readonly url: string;
			readonly emailAdmitted: boolean;
			readonly __typename: "InvitationIssued";
		};
		readonly __typename: "InvitationIssuedPayload";
	};
};

export type SettingsMembersInvitationCreate$artifact = typeof artifact

"HoudiniHash=5a1047a8074e75e66a354bcb518e9d2b0cd097d8f4c600efebbba2c5e3c89bca"

import type { InvitationBatchCreateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsMembersInvitationCreateMany",
    "kind": "HoudiniMutation",
    "hash": "0d58a48130dc3d27ec8a9efb6f0094589a50b8d21fc83ee5ff175d58b2a4d0fb",
    "raw": `mutation SettingsMembersInvitationCreateMany($invitationCreateManyInput: InvitationBatchCreateInput!) {
    invitationCreateMany(input: $invitationCreateManyInput) {
        edges {
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
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "invitationCreateMany": {
                "type": "InvitationIssuedConnection",
                "keyRaw": "invitationCreateMany(input: $invitationCreateManyInput)",

                "selection": {
                    "fields": {
                        "edges": {
                            "type": "InvitationIssuedEdge",
                            "keyRaw": "edges",

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
            "invitationCreateManyInput": "InvitationBatchCreateInput",
        },

        "types": {
            "InvitationBatchCreateInput": {
                "invitations": "InvitationCreateInput",
            },
            "InvitationCreateInput": {
                "email": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsMembersInvitationCreateMany = {
	readonly "input": SettingsMembersInvitationCreateMany$input;
	readonly "result": SettingsMembersInvitationCreateMany$result;
};

export type SettingsMembersInvitationCreateMany$result = {
	readonly invitationCreateMany: {
		readonly edges: ({
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
		})[];
	};
};

export type SettingsMembersInvitationCreateMany$input = {
	invitationCreateManyInput: InvitationBatchCreateInput;
};

export type SettingsMembersInvitationCreateMany$optimistic = {
	readonly invitationCreateMany?: {
		readonly edges?: ({
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
		})[];
	};
};

export type SettingsMembersInvitationCreateMany$unmasked = {
	readonly invitationCreateMany: {
		readonly edges: ({
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
			readonly __typename: "InvitationIssuedEdge";
		})[];
		readonly __typename: "InvitationIssuedConnection";
	};
};

export type SettingsMembersInvitationCreateMany$artifact = typeof artifact

"HoudiniHash=0d58a48130dc3d27ec8a9efb6f0094589a50b8d21fc83ee5ff175d58b2a4d0fb"

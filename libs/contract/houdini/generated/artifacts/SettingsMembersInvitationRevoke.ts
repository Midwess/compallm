const artifact = {
    "name": "SettingsMembersInvitationRevoke",
    "kind": "HoudiniMutation",
    "hash": "bdca31c896298f99cce0b6c53dca4c66449d1712fb91b5a33d3e45dacaaa1fd1",
    "raw": `mutation SettingsMembersInvitationRevoke($invitationRevokeId: ID!) {
    invitationRevoke(id: $invitationRevokeId) {
        node {
            id
            email
            status
            expiresAt
            issuedAt
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
            "invitationRevoke": {
                "type": "InvitationPayload",
                "keyRaw": "invitationRevoke(id: $invitationRevokeId)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Invitation",
                            "keyRaw": "node",

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
            "invitationRevokeId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsMembersInvitationRevoke = {
	readonly "input": SettingsMembersInvitationRevoke$input;
	readonly "result": SettingsMembersInvitationRevoke$result;
};

export type SettingsMembersInvitationRevoke$result = {
	readonly invitationRevoke: {
		readonly node: {
			readonly id: string;
			readonly email: string;
			readonly status: string;
			readonly expiresAt: number;
			readonly issuedAt: number;
		};
	};
};

export type SettingsMembersInvitationRevoke$input = {
	invitationRevokeId: string;
};

export type SettingsMembersInvitationRevoke$optimistic = {
	readonly invitationRevoke?: {
		readonly node?: {
			readonly id?: string;
			readonly email?: string;
			readonly status?: string;
			readonly expiresAt?: number;
			readonly issuedAt?: number;
		};
	};
};

export type SettingsMembersInvitationRevoke$unmasked = {
	readonly invitationRevoke: {
		readonly node: {
			readonly id: string;
			readonly email: string;
			readonly status: string;
			readonly expiresAt: number;
			readonly issuedAt: number;
			readonly __typename: "Invitation";
		};
		readonly __typename: "InvitationPayload";
	};
};

export type SettingsMembersInvitationRevoke$artifact = typeof artifact

"HoudiniHash=bdca31c896298f99cce0b6c53dca4c66449d1712fb91b5a33d3e45dacaaa1fd1"

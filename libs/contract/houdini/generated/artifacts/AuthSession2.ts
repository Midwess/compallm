const artifact = {
    "name": "AuthSession2",
    "kind": "HoudiniQuery",
    "hash": "61b50a9ad2250d3b8004ed13b489e91f30fdc6dbb1841c81d8aa6a85ee49d2bb",
    "raw": `query AuthSession2 {
    session {
        node {
            id
            userId
            organizationId
            organizationName
            avatarUrl
            ipAddress
            userAgent
            signedInAt
            lastSeenAt
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
            "session": {
                "type": "SessionPayload",
                "keyRaw": "session",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Session",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
                                        "visible": true,
                                    },

                                    "userId": {
                                        "type": "ID",
                                        "keyRaw": "userId",
                                        "visible": true,
                                    },

                                    "organizationId": {
                                        "type": "ID",
                                        "keyRaw": "organizationId",
                                        "visible": true,
                                    },

                                    "organizationName": {
                                        "type": "String",
                                        "keyRaw": "organizationName",
                                        "visible": true,
                                    },

                                    "avatarUrl": {
                                        "type": "String",
                                        "keyRaw": "avatarUrl",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "ipAddress": {
                                        "type": "String",
                                        "keyRaw": "ipAddress",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "userAgent": {
                                        "type": "String",
                                        "keyRaw": "userAgent",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "signedInAt": {
                                        "type": "String",
                                        "keyRaw": "signedInAt",
                                        "visible": true,
                                    },

                                    "lastSeenAt": {
                                        "type": "String",
                                        "keyRaw": "lastSeenAt",
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

export type AuthSession2 = {
	readonly "input"?: AuthSession2$input;
	readonly "result": AuthSession2$result | undefined;
};

export type AuthSession2$result = {
	/**
	 * Return the verified current session identity.
	 */
	readonly session: {
		readonly node: {
			/**
			 * Login session identity, serialized as a string without numeric precision loss.
			 */
			readonly id: string;
			/**
			 * User owning this verified session.
			 */
			readonly userId: string;
			/**
			 * Organization selected for this verified session.
			 */
			readonly organizationId: string;
			/**
			 * Current name of this login's organization, resolved only when selected.
			 */
			readonly organizationName: string;
			/**
			 * Public avatar for this user; clients show a fallback while absent or loading.
			 */
			readonly avatarUrl: string | null;
			/**
			 * IP observed by the API connection at sign-in; it may belong to a proxy or VPN.
			 */
			readonly ipAddress: string | null;
			/**
			 * User-Agent captured at sign-in, used only for session recognition.
			 */
			readonly userAgent: string | null;
			readonly signedInAt: string;
			readonly lastSeenAt: string;
		};
	};
};

export type AuthSession2$input = null | undefined;

export type AuthSession2$unmasked = {
	/**
	 * Return the verified current session identity.
	 */
	readonly session: {
		readonly node: {
			/**
			 * Login session identity, serialized as a string without numeric precision loss.
			 */
			readonly id: string;
			/**
			 * User owning this verified session.
			 */
			readonly userId: string;
			/**
			 * Organization selected for this verified session.
			 */
			readonly organizationId: string;
			/**
			 * Current name of this login's organization, resolved only when selected.
			 */
			readonly organizationName: string;
			/**
			 * Public avatar for this user; clients show a fallback while absent or loading.
			 */
			readonly avatarUrl: string | null;
			/**
			 * IP observed by the API connection at sign-in; it may belong to a proxy or VPN.
			 */
			readonly ipAddress: string | null;
			/**
			 * User-Agent captured at sign-in, used only for session recognition.
			 */
			readonly userAgent: string | null;
			readonly signedInAt: string;
			readonly lastSeenAt: string;
			readonly __typename: "Session";
		};
		readonly __typename: "SessionPayload";
	};
};

export type AuthSession2$artifact = typeof artifact

"HoudiniHash=61b50a9ad2250d3b8004ed13b489e91f30fdc6dbb1841c81d8aa6a85ee49d2bb"

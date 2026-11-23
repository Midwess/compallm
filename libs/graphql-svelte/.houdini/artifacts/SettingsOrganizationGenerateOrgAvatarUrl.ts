import type { ImageUploadInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsOrganizationGenerateOrgAvatarUrl",
    "kind": "HoudiniMutation",
    "hash": "d63da13d1f37aa1860cf9e27dc0da27aba138f3c249d4867d607e274f0f890c3",
    "raw": `mutation SettingsOrganizationGenerateOrgAvatarUrl($generateOrgAvatarUrlInput: ImageUploadInput!) {
    generateOrgAvatarUrl(input: $generateOrgAvatarUrlInput) {
        node {
            url
            fields {
                name
                value
                __typename
            }
            expiresAt
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
            "generateOrgAvatarUrl": {
                "type": "AvatarUploadPayload",
                "keyRaw": "generateOrgAvatarUrl(input: $generateOrgAvatarUrlInput)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "AvatarUpload",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "url": {
                                        "type": "String",
                                        "keyRaw": "url",
                                        "visible": true,
                                    },

                                    "fields": {
                                        "type": "UploadFormField",
                                        "keyRaw": "fields",

                                        "selection": {
                                            "fields": {
                                                "name": {
                                                    "type": "String",
                                                    "keyRaw": "name",
                                                    "visible": true,
                                                },

                                                "value": {
                                                    "type": "String",
                                                    "keyRaw": "value",
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

                                    "expiresAt": {
                                        "type": "String",
                                        "keyRaw": "expiresAt",
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
            "generateOrgAvatarUrlInput": "ImageUploadInput",
        },

        "types": {
            "ImageUploadInput": {
                "mediaType": "ImageMediaType",
                "size": "Int",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsOrganizationGenerateOrgAvatarUrl = {
	readonly "input": SettingsOrganizationGenerateOrgAvatarUrl$input;
	readonly "result": SettingsOrganizationGenerateOrgAvatarUrl$result;
};

export type SettingsOrganizationGenerateOrgAvatarUrl$result = {
	readonly generateOrgAvatarUrl: {
		readonly node: {
			readonly url: string;
			readonly fields: ({
				readonly name: string;
				readonly value: string;
			})[];
			/**
			 * Expiration as Unix seconds, encoded as a string.
			 */
			readonly expiresAt: string;
		};
	};
};

export type SettingsOrganizationGenerateOrgAvatarUrl$input = {
	generateOrgAvatarUrlInput: ImageUploadInput;
};

export type SettingsOrganizationGenerateOrgAvatarUrl$optimistic = {
	readonly generateOrgAvatarUrl?: {
		readonly node?: {
			readonly url?: string;
			readonly fields?: ({
				readonly name?: string;
				readonly value?: string;
			})[];
			/**
			 * Expiration as Unix seconds, encoded as a string.
			 */
			readonly expiresAt?: string;
		};
	};
};

export type SettingsOrganizationGenerateOrgAvatarUrl$unmasked = {
	readonly generateOrgAvatarUrl: {
		readonly node: {
			readonly url: string;
			readonly fields: ({
				readonly name: string;
				readonly value: string;
				readonly __typename: "UploadFormField";
			})[];
			/**
			 * Expiration as Unix seconds, encoded as a string.
			 */
			readonly expiresAt: string;
			readonly __typename: "AvatarUpload";
		};
		readonly __typename: "AvatarUploadPayload";
	};
};

export type SettingsOrganizationGenerateOrgAvatarUrl$artifact = typeof artifact

"HoudiniHash=d63da13d1f37aa1860cf9e27dc0da27aba138f3c249d4867d607e274f0f890c3"

import type { ImageUploadInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsProfileGenerateUrlAvatarUrl",
    "kind": "HoudiniMutation",
    "hash": "686bf21c292a425e8b24aa1980e9519e1618478fd6b6ef6aebd99cf41a977aed",
    "raw": `mutation SettingsProfileGenerateUrlAvatarUrl($generateUrlAvatarUrlInput: ImageUploadInput!) {
    generateUrlAvatarUrl(input: $generateUrlAvatarUrlInput) {
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
            "generateUrlAvatarUrl": {
                "type": "AvatarUploadPayload",
                "keyRaw": "generateUrlAvatarUrl(input: $generateUrlAvatarUrlInput)",

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
            "generateUrlAvatarUrlInput": "ImageUploadInput",
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

export type SettingsProfileGenerateUrlAvatarUrl = {
	readonly "input": SettingsProfileGenerateUrlAvatarUrl$input;
	readonly "result": SettingsProfileGenerateUrlAvatarUrl$result;
};

export type SettingsProfileGenerateUrlAvatarUrl$result = {
	/**
	 * Authorize a direct JPEG/PNG/WebP avatar upload up to 5 MiB for five minutes.
	 */
	readonly generateUrlAvatarUrl: {
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

export type SettingsProfileGenerateUrlAvatarUrl$input = {
	generateUrlAvatarUrlInput: ImageUploadInput;
};

export type SettingsProfileGenerateUrlAvatarUrl$optimistic = {
	/**
	 * Authorize a direct JPEG/PNG/WebP avatar upload up to 5 MiB for five minutes.
	 */
	readonly generateUrlAvatarUrl?: {
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

export type SettingsProfileGenerateUrlAvatarUrl$unmasked = {
	/**
	 * Authorize a direct JPEG/PNG/WebP avatar upload up to 5 MiB for five minutes.
	 */
	readonly generateUrlAvatarUrl: {
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

export type SettingsProfileGenerateUrlAvatarUrl$artifact = typeof artifact

"HoudiniHash=686bf21c292a425e8b24aa1980e9519e1618478fd6b6ef6aebd99cf41a977aed"

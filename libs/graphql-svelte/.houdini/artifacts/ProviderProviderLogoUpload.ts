import type { ImageUploadInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "ProviderProviderLogoUpload",
    "kind": "HoudiniMutation",
    "hash": "00af01669f5aabdd023e9b2810e5f799b6d9ab4fa378ea6a6c7a09832b8197c8",
    "raw": `mutation ProviderProviderLogoUpload($providerLogoUploadInput: ImageUploadInput!) {
    providerLogoUpload(input: $providerLogoUploadInput) {
        node {
            id
            upload {
                url
                fields {
                    name
                    value
                    __typename
                }
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
            "providerLogoUpload": {
                "type": "ProviderLogoUploadPayload",
                "keyRaw": "providerLogoUpload(input: $providerLogoUploadInput)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "ProviderLogoUpload",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "id": {
                                        "type": "String",
                                        "keyRaw": "id",
                                        "visible": true,
                                    },

                                    "upload": {
                                        "type": "AvatarUpload",
                                        "keyRaw": "upload",

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
            "providerLogoUploadInput": "ImageUploadInput",
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

export type ProviderProviderLogoUpload = {
	readonly "input": ProviderProviderLogoUpload$input;
	readonly "result": ProviderProviderLogoUpload$result;
};

export type ProviderProviderLogoUpload$result = {
	readonly providerLogoUpload: {
		readonly node: {
			readonly id: string;
			readonly upload: {
				readonly url: string;
				readonly fields: ({
					readonly name: string;
					readonly value: string;
				})[];
			};
		};
	};
};

export type ProviderProviderLogoUpload$input = {
	providerLogoUploadInput: ImageUploadInput;
};

export type ProviderProviderLogoUpload$optimistic = {
	readonly providerLogoUpload?: {
		readonly node?: {
			readonly id?: string;
			readonly upload?: {
				readonly url?: string;
				readonly fields?: ({
					readonly name?: string;
					readonly value?: string;
				})[];
			};
		};
	};
};

export type ProviderProviderLogoUpload$unmasked = {
	readonly providerLogoUpload: {
		readonly node: {
			readonly id: string;
			readonly upload: {
				readonly url: string;
				readonly fields: ({
					readonly name: string;
					readonly value: string;
					readonly __typename: "UploadFormField";
				})[];
				readonly __typename: "AvatarUpload";
			};
			readonly __typename: "ProviderLogoUpload";
		};
		readonly __typename: "ProviderLogoUploadPayload";
	};
};

export type ProviderProviderLogoUpload$artifact = typeof artifact

"HoudiniHash=00af01669f5aabdd023e9b2810e5f799b6d9ab4fa378ea6a6c7a09832b8197c8"

import type { ImageUploadInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "OnboardingGenerateUrlAvatarUrl",
    "kind": "HoudiniMutation",
    "hash": "94d49b52874b909ebdc205f09e1abe5a93ba54e59e9650e696a1954d9fbffe0e",
    "raw": `mutation OnboardingGenerateUrlAvatarUrl($generateUrlAvatarUrlInput: ImageUploadInput!) {
    generateUrlAvatarUrl(input: $generateUrlAvatarUrlInput) {
        node {
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

export type OnboardingGenerateUrlAvatarUrl = {
	readonly "input": OnboardingGenerateUrlAvatarUrl$input;
	readonly "result": OnboardingGenerateUrlAvatarUrl$result;
};

export type OnboardingGenerateUrlAvatarUrl$result = {
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
		};
	};
};

export type OnboardingGenerateUrlAvatarUrl$input = {
	generateUrlAvatarUrlInput: ImageUploadInput;
};

export type OnboardingGenerateUrlAvatarUrl$optimistic = {
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
		};
	};
};

export type OnboardingGenerateUrlAvatarUrl$unmasked = {
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
			readonly __typename: "AvatarUpload";
		};
		readonly __typename: "AvatarUploadPayload";
	};
};

export type OnboardingGenerateUrlAvatarUrl$artifact = typeof artifact

"HoudiniHash=94d49b52874b909ebdc205f09e1abe5a93ba54e59e9650e696a1954d9fbffe0e"

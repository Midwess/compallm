import type { PreferenceAppearanceMode$options, PreferenceBadgeStyle$options, PreferenceCodeTheme$options, PreferenceDarkPalette$options, PreferenceDestination$options, PreferenceFontFamily$options, PreferenceKeyCombination$options, PreferenceLightPalette$options, PreferenceNamingFormat$options, PreferencePixelFontSize$options, PreferenceRelativeFontSize$options, PreferenceSidebarCategory$options, PreferenceSidebarVisibility$options, PreferenceWeekday$options } from "$houdini/graphql/enums";
const artifact = {
    "name": "PreferenceUserUserPreferences",
    "kind": "HoudiniQuery",
    "hash": "bab9d1d1c2b40bd559679226faaa8d4958728be77ef1a341dc6f0caba33850f9",
    "raw": `query PreferenceUserUserPreferences {
    userPreferences {
        node {
            navigation {
                home
                __typename
            }
            memberDisplay {
                nameFormat
                __typename
            }
            calendar {
                firstDay
                __typename
            }
            theme {
                mode
                palettes {
                    light
                    dark
                    __typename
                }
                __typename
            }
            typography {
                family
                size
                __typename
            }
            accessibility {
                underlineLinks
                reduceMotion
                pointerCursors
                __typename
            }
            messageComposition {
                convertEmoticons
                sendShortcut
                __typename
            }
            codeAppearance {
                theme
                font {
                    family
                    size
                    __typename
                }
                __typename
            }
            sidebar {
                badgeStyle
                entries {
                    item
                    visibility
                    __typename
                }
                layout {
                    category
                    items
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

    "rootType": "Query",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "userPreferences": {
                "type": "UserPreferencesPayload",
                "keyRaw": "userPreferences",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "UserPreferences",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "navigation": {
                                        "type": "NavigationPreferences",
                                        "keyRaw": "navigation",

                                        "selection": {
                                            "fields": {
                                                "home": {
                                                    "type": "PreferenceDestination",
                                                    "keyRaw": "home",
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

                                    "memberDisplay": {
                                        "type": "MemberDisplayPreferences",
                                        "keyRaw": "memberDisplay",

                                        "selection": {
                                            "fields": {
                                                "nameFormat": {
                                                    "type": "PreferenceNamingFormat",
                                                    "keyRaw": "nameFormat",
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

                                    "calendar": {
                                        "type": "CalendarPreferences",
                                        "keyRaw": "calendar",

                                        "selection": {
                                            "fields": {
                                                "firstDay": {
                                                    "type": "PreferenceWeekday",
                                                    "keyRaw": "firstDay",
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

                                    "theme": {
                                        "type": "ThemePreferences",
                                        "keyRaw": "theme",

                                        "selection": {
                                            "fields": {
                                                "mode": {
                                                    "type": "PreferenceAppearanceMode",
                                                    "keyRaw": "mode",
                                                    "visible": true,
                                                },

                                                "palettes": {
                                                    "type": "PalettePreferences",
                                                    "keyRaw": "palettes",

                                                    "selection": {
                                                        "fields": {
                                                            "light": {
                                                                "type": "PreferenceLightPalette",
                                                                "keyRaw": "light",
                                                                "visible": true,
                                                            },

                                                            "dark": {
                                                                "type": "PreferenceDarkPalette",
                                                                "keyRaw": "dark",
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

                                    "typography": {
                                        "type": "TypographyPreferences",
                                        "keyRaw": "typography",

                                        "selection": {
                                            "fields": {
                                                "family": {
                                                    "type": "PreferenceFontFamily",
                                                    "keyRaw": "family",
                                                    "visible": true,
                                                },

                                                "size": {
                                                    "type": "PreferenceRelativeFontSize",
                                                    "keyRaw": "size",
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

                                    "accessibility": {
                                        "type": "AccessibilityPreferences",
                                        "keyRaw": "accessibility",

                                        "selection": {
                                            "fields": {
                                                "underlineLinks": {
                                                    "type": "Boolean",
                                                    "keyRaw": "underlineLinks",
                                                    "visible": true,
                                                },

                                                "reduceMotion": {
                                                    "type": "Boolean",
                                                    "keyRaw": "reduceMotion",
                                                    "visible": true,
                                                },

                                                "pointerCursors": {
                                                    "type": "Boolean",
                                                    "keyRaw": "pointerCursors",
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

                                    "messageComposition": {
                                        "type": "MessageCompositionPreferences",
                                        "keyRaw": "messageComposition",

                                        "selection": {
                                            "fields": {
                                                "convertEmoticons": {
                                                    "type": "Boolean",
                                                    "keyRaw": "convertEmoticons",
                                                    "visible": true,
                                                },

                                                "sendShortcut": {
                                                    "type": "PreferenceKeyCombination",
                                                    "keyRaw": "sendShortcut",
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

                                    "codeAppearance": {
                                        "type": "CodeAppearancePreferences",
                                        "keyRaw": "codeAppearance",

                                        "selection": {
                                            "fields": {
                                                "theme": {
                                                    "type": "PreferenceCodeTheme",
                                                    "keyRaw": "theme",
                                                    "visible": true,
                                                },

                                                "font": {
                                                    "type": "CodeFontPreferences",
                                                    "keyRaw": "font",

                                                    "selection": {
                                                        "fields": {
                                                            "family": {
                                                                "type": "PreferenceFontFamily",
                                                                "keyRaw": "family",
                                                                "visible": true,
                                                            },

                                                            "size": {
                                                                "type": "PreferencePixelFontSize",
                                                                "keyRaw": "size",
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

                                    "sidebar": {
                                        "type": "SidebarPreferences",
                                        "keyRaw": "sidebar",

                                        "selection": {
                                            "fields": {
                                                "badgeStyle": {
                                                    "type": "PreferenceBadgeStyle",
                                                    "keyRaw": "badgeStyle",
                                                    "visible": true,
                                                },

                                                "entries": {
                                                    "type": "SidebarEntry",
                                                    "keyRaw": "entries",

                                                    "selection": {
                                                        "fields": {
                                                            "item": {
                                                                "type": "PreferenceDestination",
                                                                "keyRaw": "item",
                                                                "visible": true,
                                                            },

                                                            "visibility": {
                                                                "type": "PreferenceSidebarVisibility",
                                                                "keyRaw": "visibility",
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

                                                "layout": {
                                                    "type": "SidebarLayoutSection",
                                                    "keyRaw": "layout",

                                                    "selection": {
                                                        "fields": {
                                                            "category": {
                                                                "type": "PreferenceSidebarCategory",
                                                                "keyRaw": "category",
                                                                "visible": true,
                                                            },

                                                            "items": {
                                                                "type": "PreferenceDestination",
                                                                "keyRaw": "items",
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
    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type PreferenceUserUserPreferences = {
	readonly "input"?: PreferenceUserUserPreferences$input;
	readonly "result": PreferenceUserUserPreferences$result | undefined;
};

export type PreferenceUserUserPreferences$result = {
	/**
	 * Return the signed-in user's personal preferences; missing rows use Core defaults without a write.
	 */
	readonly userPreferences: {
		readonly node: {
			readonly navigation: {
				readonly home: PreferenceDestination$options;
			};
			readonly memberDisplay: {
				readonly nameFormat: PreferenceNamingFormat$options;
			};
			readonly calendar: {
				readonly firstDay: PreferenceWeekday$options;
			};
			readonly theme: {
				readonly mode: PreferenceAppearanceMode$options;
				readonly palettes: {
					readonly light: PreferenceLightPalette$options;
					readonly dark: PreferenceDarkPalette$options;
				};
			};
			readonly typography: {
				readonly family: PreferenceFontFamily$options;
				readonly size: PreferenceRelativeFontSize$options;
			};
			readonly accessibility: {
				readonly underlineLinks: boolean;
				readonly reduceMotion: boolean;
				readonly pointerCursors: boolean;
			};
			readonly messageComposition: {
				readonly convertEmoticons: boolean;
				readonly sendShortcut: PreferenceKeyCombination$options;
			};
			readonly codeAppearance: {
				readonly theme: PreferenceCodeTheme$options;
				readonly font: {
					readonly family: PreferenceFontFamily$options;
					readonly size: PreferencePixelFontSize$options;
				};
			};
			readonly sidebar: {
				readonly badgeStyle: PreferenceBadgeStyle$options;
				readonly entries: ({
					readonly item: PreferenceDestination$options;
					readonly visibility: PreferenceSidebarVisibility$options;
				})[];
				readonly layout: ({
					readonly category: PreferenceSidebarCategory$options;
					readonly items: (PreferenceDestination$options)[];
				})[];
			};
		};
	};
};

export type PreferenceUserUserPreferences$input = null | undefined;

export type PreferenceUserUserPreferences$unmasked = {
	/**
	 * Return the signed-in user's personal preferences; missing rows use Core defaults without a write.
	 */
	readonly userPreferences: {
		readonly node: {
			readonly navigation: {
				readonly home: PreferenceDestination$options;
				readonly __typename: "NavigationPreferences";
			};
			readonly memberDisplay: {
				readonly nameFormat: PreferenceNamingFormat$options;
				readonly __typename: "MemberDisplayPreferences";
			};
			readonly calendar: {
				readonly firstDay: PreferenceWeekday$options;
				readonly __typename: "CalendarPreferences";
			};
			readonly theme: {
				readonly mode: PreferenceAppearanceMode$options;
				readonly palettes: {
					readonly light: PreferenceLightPalette$options;
					readonly dark: PreferenceDarkPalette$options;
					readonly __typename: "PalettePreferences";
				};
				readonly __typename: "ThemePreferences";
			};
			readonly typography: {
				readonly family: PreferenceFontFamily$options;
				readonly size: PreferenceRelativeFontSize$options;
				readonly __typename: "TypographyPreferences";
			};
			readonly accessibility: {
				readonly underlineLinks: boolean;
				readonly reduceMotion: boolean;
				readonly pointerCursors: boolean;
				readonly __typename: "AccessibilityPreferences";
			};
			readonly messageComposition: {
				readonly convertEmoticons: boolean;
				readonly sendShortcut: PreferenceKeyCombination$options;
				readonly __typename: "MessageCompositionPreferences";
			};
			readonly codeAppearance: {
				readonly theme: PreferenceCodeTheme$options;
				readonly font: {
					readonly family: PreferenceFontFamily$options;
					readonly size: PreferencePixelFontSize$options;
					readonly __typename: "CodeFontPreferences";
				};
				readonly __typename: "CodeAppearancePreferences";
			};
			readonly sidebar: {
				readonly badgeStyle: PreferenceBadgeStyle$options;
				readonly entries: ({
					readonly item: PreferenceDestination$options;
					readonly visibility: PreferenceSidebarVisibility$options;
					readonly __typename: "SidebarEntry";
				})[];
				readonly layout: ({
					readonly category: PreferenceSidebarCategory$options;
					readonly items: (PreferenceDestination$options)[];
					readonly __typename: "SidebarLayoutSection";
				})[];
				readonly __typename: "SidebarPreferences";
			};
			readonly __typename: "UserPreferences";
		};
		readonly __typename: "UserPreferencesPayload";
	};
};

export type PreferenceUserUserPreferences$artifact = typeof artifact

"HoudiniHash=bab9d1d1c2b40bd559679226faaa8d4958728be77ef1a341dc6f0caba33850f9"

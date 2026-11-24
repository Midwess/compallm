import type { PreferenceAppearanceMode$options, PreferenceBadgeStyle$options, PreferenceCodeTheme$options, PreferenceDarkPalette$options, PreferenceDestination$options, PreferenceFontFamily$options, PreferenceKeyCombination$options, PreferenceLightPalette$options, PreferenceNamingFormat$options, PreferencePixelFontSize$options, PreferenceRelativeFontSize$options, PreferenceSidebarCategory$options, PreferenceSidebarVisibility$options, PreferenceWeekday$options } from "$houdini/graphql/enums";
import type { UserPreferencesPatchInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "PreferenceUserUserPreferencesUpdate",
    "kind": "HoudiniMutation",
    "hash": "0cf0f66f7809049e1787cbc617d2f489ef5d40c1f02ff80d4ba13689d2c75c20",
    "raw": `mutation PreferenceUserUserPreferencesUpdate($includeAccessibility: Boolean! = false, $includeCalendar: Boolean! = false, $includeCodeAppearance: Boolean! = false, $includeMemberDisplay: Boolean! = false, $includeMessageComposition: Boolean! = false, $includeNavigation: Boolean! = false, $includeSidebar: Boolean! = false, $includeTheme: Boolean! = false, $includeTypography: Boolean! = false, $userPreferencesUpdateInput: UserPreferencesPatchInput!) {
    userPreferencesUpdate(input: $userPreferencesUpdateInput) {
        node {
            navigation @include(if: $includeNavigation) {
                home
                __typename
            }
            memberDisplay @include(if: $includeMemberDisplay) {
                nameFormat
                __typename
            }
            calendar @include(if: $includeCalendar) {
                firstDay
                __typename
            }
            theme @include(if: $includeTheme) {
                mode
                palettes {
                    light
                    dark
                    __typename
                }
                __typename
            }
            typography @include(if: $includeTypography) {
                family
                size
                __typename
            }
            accessibility @include(if: $includeAccessibility) {
                underlineLinks
                reduceMotion
                pointerCursors
                __typename
            }
            messageComposition @include(if: $includeMessageComposition) {
                convertEmoticons
                sendShortcut
                __typename
            }
            codeAppearance @include(if: $includeCodeAppearance) {
                theme
                font {
                    family
                    size
                    __typename
                }
                __typename
            }
            sidebar @include(if: $includeSidebar) {
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

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "userPreferencesUpdate": {
                "type": "UserPreferencesPayload",
                "keyRaw": "userPreferencesUpdate(input: $userPreferencesUpdateInput)",

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

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeNavigation",
                                                    }
                                                }
                                            }
                                        }],


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

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeMemberDisplay",
                                                    }
                                                }
                                            }
                                        }],


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

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeCalendar",
                                                    }
                                                }
                                            }
                                        }],


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

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeTheme",
                                                    }
                                                }
                                            }
                                        }],


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

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeTypography",
                                                    }
                                                }
                                            }
                                        }],


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

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeAccessibility",
                                                    }
                                                }
                                            }
                                        }],


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

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeMessageComposition",
                                                    }
                                                }
                                            }
                                        }],


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

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeCodeAppearance",
                                                    }
                                                }
                                            }
                                        }],


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

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeSidebar",
                                                    }
                                                }
                                            }
                                        }],


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

    "input": {
        "fields": {
            "userPreferencesUpdateInput": "UserPreferencesPatchInput",
            "includeNavigation": "Boolean",
            "includeMemberDisplay": "Boolean",
            "includeCalendar": "Boolean",
            "includeTheme": "Boolean",
            "includeTypography": "Boolean",
            "includeAccessibility": "Boolean",
            "includeMessageComposition": "Boolean",
            "includeCodeAppearance": "Boolean",
            "includeSidebar": "Boolean",
        },

        "types": {
            "AccessibilityPatchInput": {
                "pointerCursors": "Boolean",
                "reduceMotion": "Boolean",
                "underlineLinks": "Boolean",
            },
            "CalendarPatchInput": {
                "firstDay": "PreferenceWeekday",
            },
            "CodeAppearancePatchInput": {
                "font": "CodeFontPatchInput",
                "theme": "PreferenceCodeTheme",
            },
            "CodeFontPatchInput": {
                "family": "PreferenceFontFamily",
                "size": "PreferencePixelFontSize",
            },
            "MemberDisplayPatchInput": {
                "nameFormat": "PreferenceNamingFormat",
            },
            "MessageCompositionPatchInput": {
                "convertEmoticons": "Boolean",
                "sendShortcut": "PreferenceKeyCombination",
            },
            "NavigationPatchInput": {
                "home": "PreferenceDestination",
            },
            "PalettePatchInput": {
                "dark": "PreferenceDarkPalette",
                "light": "PreferenceLightPalette",
            },
            "SidebarLayoutSectionInput": {
                "category": "PreferenceSidebarCategory",
                "items": "PreferenceDestination",
            },
            "SidebarPatchInput": {
                "badgeStyle": "PreferenceBadgeStyle",
                "layout": "SidebarLayoutSectionInput",
                "order": "PreferenceDestination",
                "visibility": "SidebarVisibilityPatchInput",
            },
            "SidebarVisibilityPatchInput": {
                "item": "PreferenceDestination",
                "visibility": "PreferenceSidebarVisibility",
            },
            "ThemePatchInput": {
                "mode": "PreferenceAppearanceMode",
                "palettes": "PalettePatchInput",
            },
            "TypographyPatchInput": {
                "family": "PreferenceFontFamily",
                "size": "PreferenceRelativeFontSize",
            },
            "UserPreferencesPatchInput": {
                "accessibility": "AccessibilityPatchInput",
                "calendar": "CalendarPatchInput",
                "codeAppearance": "CodeAppearancePatchInput",
                "memberDisplay": "MemberDisplayPatchInput",
                "messageComposition": "MessageCompositionPatchInput",
                "navigation": "NavigationPatchInput",
                "sidebar": "SidebarPatchInput",
                "theme": "ThemePatchInput",
                "typography": "TypographyPatchInput",
            },
        },

        "defaults": {
            "includeNavigation": false,
            "includeMemberDisplay": false,
            "includeCalendar": false,
            "includeTheme": false,
            "includeTypography": false,
            "includeAccessibility": false,
            "includeMessageComposition": false,
            "includeCodeAppearance": false,
            "includeSidebar": false,
        },

        "runtimeScalars": {},
    },

} as const

export default artifact

export type PreferenceUserUserPreferencesUpdate = {
	readonly "input": PreferenceUserUserPreferencesUpdate$input;
	readonly "result": PreferenceUserUserPreferencesUpdate$result;
};

export type PreferenceUserUserPreferencesUpdate$result = {
	/**
	 * Atomically patch the signed-in user's preferences, preserving omitted fields.
	 */
	readonly userPreferencesUpdate: {
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

export type PreferenceUserUserPreferencesUpdate$input = {
	userPreferencesUpdateInput: UserPreferencesPatchInput;
	includeNavigation: boolean;
	includeMemberDisplay: boolean;
	includeCalendar: boolean;
	includeTheme: boolean;
	includeTypography: boolean;
	includeAccessibility: boolean;
	includeMessageComposition: boolean;
	includeCodeAppearance: boolean;
	includeSidebar: boolean;
};

export type PreferenceUserUserPreferencesUpdate$optimistic = {
	/**
	 * Atomically patch the signed-in user's preferences, preserving omitted fields.
	 */
	readonly userPreferencesUpdate?: {
		readonly node?: {
			readonly navigation?: {
				readonly home?: PreferenceDestination$options;
			};
			readonly memberDisplay?: {
				readonly nameFormat?: PreferenceNamingFormat$options;
			};
			readonly calendar?: {
				readonly firstDay?: PreferenceWeekday$options;
			};
			readonly theme?: {
				readonly mode?: PreferenceAppearanceMode$options;
				readonly palettes?: {
					readonly light?: PreferenceLightPalette$options;
					readonly dark?: PreferenceDarkPalette$options;
				};
			};
			readonly typography?: {
				readonly family?: PreferenceFontFamily$options;
				readonly size?: PreferenceRelativeFontSize$options;
			};
			readonly accessibility?: {
				readonly underlineLinks?: boolean;
				readonly reduceMotion?: boolean;
				readonly pointerCursors?: boolean;
			};
			readonly messageComposition?: {
				readonly convertEmoticons?: boolean;
				readonly sendShortcut?: PreferenceKeyCombination$options;
			};
			readonly codeAppearance?: {
				readonly theme?: PreferenceCodeTheme$options;
				readonly font?: {
					readonly family?: PreferenceFontFamily$options;
					readonly size?: PreferencePixelFontSize$options;
				};
			};
			readonly sidebar?: {
				readonly badgeStyle?: PreferenceBadgeStyle$options;
				readonly entries?: ({
					readonly item?: PreferenceDestination$options;
					readonly visibility?: PreferenceSidebarVisibility$options;
				})[];
				readonly layout?: ({
					readonly category?: PreferenceSidebarCategory$options;
					readonly items?: (PreferenceDestination$options)[];
				})[];
			};
		};
	};
};

export type PreferenceUserUserPreferencesUpdate$unmasked = {
	/**
	 * Atomically patch the signed-in user's preferences, preserving omitted fields.
	 */
	readonly userPreferencesUpdate: {
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

export type PreferenceUserUserPreferencesUpdate$artifact = typeof artifact

"HoudiniHash=0cf0f66f7809049e1787cbc617d2f489ef5d40c1f02ff80d4ba13689d2c75c20"

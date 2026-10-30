import { createI18n } from "@krizaka/i18n";
import { createI18nReact } from "@krizaka/i18n/react";

import en from "./en.json";
import fr from "./fr.json";

export type Locale = "fr" | "en";
export type Messages = typeof fr;
/** French is the reference of a Krizaka product made in Montréal: en.json has the same keys (tested). */
export const i18n = createI18n<Messages, Locale>({ fr, en }, { defaultLocale: "fr" });
export const { I18nProvider, useI18n } = createI18nReact(i18n);

/** The language the app speaks: the system's, French first. The API answers in it too (`Accept-Language`). */
export const currentLocale = (): Locale => (navigator.language.startsWith("en") ? "en" : "fr");

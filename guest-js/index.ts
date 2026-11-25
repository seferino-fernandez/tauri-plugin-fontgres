import { invoke } from "@tauri-apps/api/core";

export const COMMAND = {
  GET_FACES: "plugin:fontgres|get_faces",
  GET_FAMILIES: "plugin:fontgres|get_families",
  QUERY_FACE: "plugin:fontgres|query_face",
  RELOAD: "plugin:fontgres|reload",
} as const;

/** A face's style, as its CSS keyword. */
export type Style = "normal" | "italic" | "oblique";

/** A face's stretch (CSS `font-stretch`), as its CSS keyword. */
export type Stretch =
  | "ultra-condensed"
  | "extra-condensed"
  | "condensed"
  | "semi-condensed"
  | "normal"
  | "semi-expanded"
  | "expanded"
  | "extra-expanded"
  | "ultra-expanded";

/** One font face: one style of a typeface, such as Helvetica Bold. */
export interface Face {
  /** The face's primary family name (English US when the font has one). */
  family: string;
  /** Every family name the face carries, in every language, primary first. */
  families: string[];
  /** The name that tells the face from the others of its family, such as `Bold Italic`; empty when the font has none. */
  subfamily: string;
  /** The face's PostScript name. */
  postScriptName: string;
  style: Style;
  /** CSS weight, from 1 to 1000; 400 is normal and 700 bold. */
  weight: number;
  stretch: Stretch;
  /** Whether every glyph has the same advance width. */
  monospaced: boolean;
  /** The file the face was loaded from, if any. */
  path: string | null;
  /** The face's index within its file (non-zero only in font collections). */
  index: number;
}

/** A CSS-style font query. */
export interface FaceQuery {
  /**
   * Families in priority order, as in CSS `font-family`. Names match ignoring
   * ASCII case; `serif`, `sans-serif`, `cursive`, `fantasy` and `monospace`
   * resolve to one family each.
   */
  families: string[];
  /** Wanted weight, from 1 to 1000. Defaults to 400. */
  weight?: number;
  /** Wanted style. Defaults to `normal`. */
  style?: Style;
  /** Wanted stretch. Defaults to `normal`. */
  stretch?: Stretch;
}

/**
 * Why a command failed. A command's own failures reject with this shape;
 * Tauri rejects invalid arguments and missing permissions with a plain string.
 */
export interface FontgresError {
  kind: "noFamilies" | "weightOutOfRange" | "noMatch" | "unsupportedPlatform";
  message: string;
}

/** Whether `error`, as a command rejected with it, is a {@link FontgresError}. */
export const isFontgresError = (error: unknown): error is FontgresError =>
  typeof error === "object" &&
  error !== null &&
  typeof (error as FontgresError).kind === "string" &&
  typeof (error as FontgresError).message === "string";

/**
 * Lists the faces installed on the system, sorted by family, then weight,
 * style, stretch and PostScript name.
 *
 * @param options.family Only the faces of this family, matched ignoring ASCII case.
 *
 * @example
 * import { getFaces } from "tauri-plugin-fontgres-api";
 *
 * const faces = await getFaces();
 * const helvetica = await getFaces({ family: "Helvetica" });
 */
export const getFaces = (options: { family?: string } = {}) =>
  invoke<Face[]>(COMMAND.GET_FACES, { family: options.family ?? null });

/**
 * Lists the family names installed on the system, sorted.
 *
 * @example
 * import { getFamilies } from "tauri-plugin-fontgres-api";
 *
 * const families = await getFamilies();
 */
export const getFamilies = () => invoke<string[]>(COMMAND.GET_FAMILIES);

/**
 * Returns the face that best matches a CSS-style query: the first family that
 * has a face decides, and among its faces the best stretch, style and weight
 * wins, as in CSS font matching.
 *
 * Rejects with a {@link FontgresError}: `noMatch` when no listed family is
 * installed, `noFamilies` when none is listed, `weightOutOfRange` when the
 * weight is not from 1 to 1000.
 *
 * @example
 * import { queryFace } from "tauri-plugin-fontgres-api";
 *
 * const face = await queryFace({ families: ["Helvetica Neue", "sans-serif"], weight: 700 });
 */
export const queryFace = (query: FaceQuery) =>
  invoke<Face>(COMMAND.QUERY_FACE, { query });

/**
 * Reads the system fonts again, so fonts installed or removed while the app
 * runs show up. The fonts are otherwise read once, on first use.
 */
export const reload = () => invoke<void>(COMMAND.RELOAD);

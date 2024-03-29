import * as settings from "./settings";

export const copyToClipboard = async (text: string) => {
    navigator.clipboard.writeText(text);
}

export const copyIfEnabled = async (text: string) => {
    if (await settings.get<boolean>('copyToClipboard')) {
        await copyToClipboard(text);
    }
}
import { invoke } from "@tauri-apps/api/tauri";

export interface ClipboardEntry {
    id: number;
    type: number;
    path: string;
    content: string;
    timestamp: number;
    hash: string;
}

export interface ExpiredConfig {
    text: number;
    img: number;
    file: number;
}
export interface PreviewConfig {
    preview_number: number;
}

export interface UserConfig {
    expired_config: ExpiredConfig;
    preview_config: PreviewConfig;
}

export class ClipboardHelper {
    static async getClipboardEntries(
        num: number = 10,
        typeList: number[] | null = null
    ): Promise<ClipboardEntry[]> {
        try {
            const result = await invoke<ClipboardEntry[]>(
                "rs_invoke_get_clipboards",
                {
                    num,
                    typeList,
                }
            );
            console.log(result);
            return result;
        } catch (error) {
            console.error("Failed to get clipboard entries:", error);
            // throw error;
            return [];
        }
    }

    static async searchClipboardEntries(
        query: string,
        num: number = 10,
        typeList: number[] | null = null
    ): Promise<ClipboardEntry[]> {
        try {
            const result = await invoke<ClipboardEntry[]>(
                "rs_invoke_search_clipboards",
                {
                    query,
                    num,
                    typeList,
                }
            );
            return result;
        } catch (error) {
            console.error("Failed to search clipboard entries:", error);
            // throw error;
            return [];
        }
    }

    static async setClipboardEntriy(
        item: ClipboardEntry
    ): Promise<void> {
        try {
            await await invoke<ClipboardEntry[]>("rs_invoke_set_clipboards", {
                item,
            });
        } catch (error) {
            console.error("setClipboardEntriy error:", error);
            throw error;
        }
    }
}

export class UserConfig {
    static async getUserConfig(): Promise<UserConfig> {
        try {
            const result = await invoke<UserConfig>("rs_invoke_get_user_config");
            return result;
        } catch (error) {
            console.error("Failed to get user config:", error);
            throw error;
        }
    }

    static async setUserConfig(userConfig: UserConfig): Promise<void> {
        try {
            await invoke<void>("rs_invoke_set_user_config", {
                userConfig,
            });
        } catch (error) {
            console.error("Failed to set user config:", error);
            throw error;
        }
    }
}

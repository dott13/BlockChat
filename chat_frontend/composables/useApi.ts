export const useApi = () => {
    const config = useRuntimeConfig();
    const baseUrl = config.public.apiBaseUrl;

    const get = async (endpoint: string) => {
        const {data, error} = await useFetch(`${baseUrl}${endpoint}`, {method: 'GET'});
        return {data, error}
    }

    const post = async (endpoint: string, body: any) => {
        const { data, error } = await useFetch(`${baseUrl}${endpoint}`, {
            method: 'POST',
            body
        });
        return { data, error };
    };

    return {get, post}
};
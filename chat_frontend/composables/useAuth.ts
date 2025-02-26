import { useApi } from "#imports";

export const useAuth = () => {
    const {post} = useApi();

    const login = async (credentials: { username: string; password: string }) => {
        const { data, error } = await post('/users/login', credentials);
        return { data, error };
    };
    
    const register = async (user: { first_name: string; last_name: string; username: string; password: string }) => {
        const { data, error } = await post('/users/register', user);
        return { data, error };
    };
    
    return { login, register };
}
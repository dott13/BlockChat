export const getToken = (): string | null => {
    return localStorage.getItem('token');
};

export const saveToken = (token: string) => {
    localStorage.setItem('token', token);
};

export const removeToken = () => {
    localStorage.removeItem('token');
};

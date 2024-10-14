export default async function myFetch(url: string, options?: RequestInit): Promise<any> {
    return fetch(url, {...options, credentials: "include"});
}
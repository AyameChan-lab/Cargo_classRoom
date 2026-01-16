import { Brawler } from "../_models/brawler";

const _default_avatar = '/assets/images/default-avatar.png';
export const defaultAvatar = (user:Brawler):string => {
    if(user.avatar_url) return user.avatar_url;
    return _default_avatar;
}
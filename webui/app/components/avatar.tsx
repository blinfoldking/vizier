
import BoringAvatar from "boring-avatars";

const Avatar = ({ name, rounded }: { name: string, rounded: boolean }) =>
  <BoringAvatar 
    className={`${rounded ? 'rounded-full' : 'rounded-xl'}`} 
    name={name} 
    variant='beam' 
    colors={["#10B981", "#14B8A6", "#059669", "#047857", "#10B981"]} 
    square 
  />;

export default Avatar

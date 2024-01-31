import CurrentUserInformation from "./CurrentUserInformation";
import EditProfilePageForm from "./EditProfilePageForm";

export default function ProfilePage() {
    return (
        <div className="w-full max-w-7xl mx-auto">
            <CurrentUserInformation />
            <EditProfilePageForm />
        </div>
    );
}

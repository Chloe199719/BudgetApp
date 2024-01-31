"use client";
import { z } from "zod";
import { Button } from "../ui/button";
import { Input } from "../ui/input";
import { Label } from "../ui/label";
import { Textarea } from "../ui/textarea";
import { useForm } from "react-hook-form";
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Popover, PopoverContent, PopoverTrigger } from "../ui/popover";
import { cn } from "@/lib/utils";
import { format } from "date-fns";
import { CalendarIcon } from "@radix-ui/react-icons";
import { Calendar } from "../ui/calendar";
import { useDispatch, useSelector } from "@/lib/redux/store";
import { postEditProfile } from "@/lib/api/user/editProfile";
import { useMutation } from "react-query";
import { useToast } from "../ui/use-toast";
import { login } from "@/lib/redux/slices/auth";
const formSchema = z.object({
    displayName: z.string().min(2),
    avatar: z
        .instanceof(File)
        .refine((file) => file.size <= 1024 * 1024, {
            // Limit file size to 1MB
            message: "File size should be less than 1MB",
        })
        .refine(
            (file) => file.type === "image/jpeg" || file.type === "image/png",
            {
                // Check file type
                message: "Unsupported file format",
            },
        )
        .optional()
        .nullable(),
    birthdate: z.date().optional().nullable(),
    aboutme: z.string().optional(),
    pronouns: z.string().optional(),
    githubLink: z.string().optional(),
    phoneNumber: z.string().optional(),
});
export type ProfilePageFormType = z.infer<typeof formSchema>;
type Props = {};
function EditProfilePageForm({}: Props) {
    const auth = useSelector((state) => state.auth);
    const { toast } = useToast();
    const dispatch = useDispatch();
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            displayName: auth.isAuthenticated ? auth.display_name : "",
            avatar: null,
            birthdate: auth.isAuthenticated
                ? auth.profile.birth_date
                    ? new Date(auth.profile.birth_date)
                    : undefined
                : undefined,
            aboutme: auth.isAuthenticated ? auth.profile.about_me || "" : "",
            pronouns: auth.isAuthenticated ? auth.profile.pronouns || "" : "",
            githubLink: auth.isAuthenticated
                ? auth.profile.github_link || ""
                : "",
            phoneNumber: auth.isAuthenticated
                ? auth.profile.phone_number || ""
                : "",
        },
    });
    const EditProfileMutation = useMutation(postEditProfile, {
        onSuccess: (data) => {
            toast({
                title: "Profile Updated",
                description: "Your profile has been updated",
            });
            dispatch(login({ ...data, isAuthenticated: true }));
        },
        onError: (error: any) => {
            toast({
                title: "Error",
                description: error.message,
            });
            console.log(error);
        },
    });

    async function onsubmit(e: z.infer<typeof formSchema>) {
        const fromData = new FormData();
        fromData.append("display_name", e.displayName);
        if (e.birthdate)
            fromData.append("birth_date", e.birthdate.toISOString());
        if (e.aboutme) fromData.append("about_me", e.aboutme);
        if (e.pronouns) fromData.append("pronouns", e.pronouns);
        if (e.githubLink) fromData.append("github_link", e.githubLink);
        if (e.phoneNumber) fromData.append("phone_number", e.phoneNumber);
        if (e.avatar) fromData.append("avatar", e.avatar);
        EditProfileMutation.mutate(fromData);
    }

    return (
        <Form {...form}>
            <form
                onSubmit={form.handleSubmit(onsubmit)}
                className="space-y-8 bg-white dark:bg-gray-900 p-8 rounded-lg shadow-md"
            >
                <FormField
                    control={form.control}
                    name="displayName"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel
                                className="text-gray-900 dark:text-gray-100"
                                htmlFor="email"
                            >
                                Display Name
                            </FormLabel>
                            <FormControl>
                                <Input
                                    className="bg-gray-100 dark:bg-gray-700"
                                    id="display-name"
                                    placeholder="Enter your display name"
                                    {...field}
                                />
                            </FormControl>

                            <FormMessage />
                        </FormItem>
                    )}
                />
                <FormField
                    control={form.control}
                    name="avatar"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel
                                className="text-gray-900 dark:text-gray-100"
                                htmlFor="email"
                            >
                                Avatar Image
                            </FormLabel>
                            <FormControl>
                                <Input
                                    className="bg-gray-100 dark:bg-gray-700"
                                    id="avatar"
                                    type="file"
                                    placeholder="Enter your display name"
                                    onBlur={field.onBlur}
                                    disabled={field.disabled}
                                    ref={field.ref}
                                    onChange={(e) => {
                                        field.onChange(e.target.files![0]);
                                    }}
                                />
                            </FormControl>

                            <FormMessage />
                        </FormItem>
                    )}
                />

                <FormField
                    control={form.control}
                    name="birthdate"
                    render={({ field }) => (
                        <FormItem className="flex flex-col w-full">
                            <FormLabel>Date of birth</FormLabel>
                            <Popover>
                                <PopoverTrigger asChild>
                                    <FormControl>
                                        <Button
                                            variant={"outline"}
                                            className={cn(
                                                "pl-3 text-left font-normal text-gray-900 dark:text-gray-100 bg-gray-100 dark:bg-gray-700 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring  disabled:cursor-not-allowed disabled:opacity-50",
                                                !field.value &&
                                                    "text-muted-foreground",
                                            )}
                                        >
                                            {field.value ? (
                                                format(field.value, "PPP")
                                            ) : (
                                                <span>Pick a date</span>
                                            )}
                                            <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                                        </Button>
                                    </FormControl>
                                </PopoverTrigger>
                                <PopoverContent
                                    className="w-auto p-0"
                                    align="start"
                                >
                                    <Calendar
                                        mode="single"
                                        selected={field.value!}
                                        onSelect={field.onChange}
                                        disabled={(date) =>
                                            date > new Date() ||
                                            date < new Date("1900-01-01")
                                        }
                                        initialFocus
                                    />
                                </PopoverContent>
                            </Popover>
                            <FormDescription>
                                Your date of birth is used to calculate your
                                age.
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <FormField
                    control={form.control}
                    name="aboutme"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel
                                className="text-gray-900 dark:text-gray-100"
                                htmlFor="email"
                            >
                                About Me
                            </FormLabel>
                            <FormControl>
                                <Textarea
                                    className="min-h-[200px] bg-gray-100 dark:bg-gray-700"
                                    id="about-me"
                                    placeholder="Tell us about yourself"
                                    {...field}
                                />
                            </FormControl>

                            <FormMessage />
                        </FormItem>
                    )}
                />
                <FormField
                    control={form.control}
                    name="pronouns"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel
                                className="text-gray-900 dark:text-gray-100"
                                htmlFor="pronouns"
                            >
                                Pronouns
                            </FormLabel>
                            <FormControl>
                                <Input
                                    className="bg-gray-100 dark:bg-gray-700"
                                    id="pronouns"
                                    placeholder="Enter your pronouns"
                                    {...field}
                                />
                            </FormControl>

                            <FormMessage />
                        </FormItem>
                    )}
                />
                <FormField
                    control={form.control}
                    name="githubLink"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel
                                className="text-gray-900 dark:text-gray-100"
                                htmlFor="github-link"
                            >
                                GitHub Link
                            </FormLabel>
                            <FormControl>
                                <Input
                                    className="bg-gray-100 dark:bg-gray-700"
                                    id="github-link"
                                    placeholder="Enter your GitHub profile link"
                                    {...field}
                                />
                            </FormControl>

                            <FormMessage />
                        </FormItem>
                    )}
                />
                <FormField
                    control={form.control}
                    name="phoneNumber"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel
                                className="text-gray-900 dark:text-gray-100"
                                htmlFor="phone-number"
                            >
                                Phone Number
                            </FormLabel>
                            <FormControl>
                                <Input
                                    className="bg-gray-100 dark:bg-gray-700"
                                    id="phone-number"
                                    placeholder="Enter your phone number"
                                    type="tel"
                                    {...field}
                                />
                            </FormControl>

                            <FormMessage />
                        </FormItem>
                    )}
                />

                <Button
                    className="w-full bg-blue-500 hover:bg-blue-700 text-white"
                    type="submit"
                    disabled={EditProfileMutation.isLoading}
                >
                    Save Changes
                </Button>
            </form>
        </Form>
    );
}
export default EditProfilePageForm;

"use client";
import { zodResolver } from "@hookform/resolvers/zod";
import { useDispatch } from "@/lib/redux/store";
import { login } from "@/lib/redux/slices/auth";
import { z } from "zod";
import { useForm } from "react-hook-form";
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "../../ui/form";
import { Input } from "../../ui/input";
import { Button } from "../../ui/button";
import { APP_NAME } from "@/lib/constants";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useMutation } from "react-query";
import { PostLogin } from "@/lib/api/auth/login";
import { useToast } from "../../ui/use-toast";

const formSchema = z.object({
    email: z.string().email(),
    password: z.string().min(2),
});
export type LoginFormType = z.infer<typeof formSchema>;
function LoginForm() {
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            email: "",
            password: "",
        },
    });
    const router = useRouter();
    const dispatch = useDispatch();
    const { toast } = useToast();

    const handleSubmitLogin = useMutation(PostLogin, {
        onSuccess: (data) => {
            dispatch(login({ ...data, isAuthenticated: true }));
            router.push("/");
        },
        onError: (error: any) => {
            form.setError("email", {
                type: "manual",
                message: error.error,
            });
            form.setError("password", {
                type: "manual",
                message: error.error,
            });
            toast({
                title: "Error",
                description: error.error,
            });
        },
    });

    function onsubmit(e: z.infer<typeof formSchema>) {
        handleSubmitLogin.mutate(e);
    }
    return (
        <div className="w-full col-span-2  px-8 h-full flex items-center justify-center flex-col gap-8 max-w-xl">
            <h2 className="text-4xl text-pretty font-bold ">
                Sign in to your{" "}
                <span className="text-blue-500"> {APP_NAME} </span> Account
            </h2>
            <h3 className="text-start w-full text-lg">Hi, Welcome Back.👋 </h3>
            <div className="w-full flex flex-col">
                <Form {...form}>
                    <form
                        onSubmit={form.handleSubmit(onsubmit)}
                        className="space-y-4 bg-blue-200 p-8 rounded-md dark:bg-gray-800"
                    >
                        <h3 className="text-center text-xl">Login</h3>
                        <FormField
                            control={form.control}
                            name="email"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel htmlFor="email">Email</FormLabel>
                                    <FormControl>
                                        <Input
                                            id="email"
                                            placeholder="chloe@example.com"
                                            {...field}
                                        />
                                    </FormControl>

                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name="password"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel htmlFor="password">
                                        Password
                                    </FormLabel>
                                    <FormControl>
                                        <Input
                                            id="password"
                                            type="password"
                                            placeholder="Password"
                                            {...field}
                                        />
                                    </FormControl>

                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <div className="flex justify-end">
                            <Link
                                href="/auth/forgot-password"
                                className="text-blue-500"
                            >
                                Forgot Password?
                            </Link>
                        </div>
                        <Button
                            disabled={handleSubmitLogin.isLoading}
                            className={`w-full disabled:bg-gray-500 disabled:cursor-not-allowed disabled:dark:bg-gray-800`}
                            type="submit"
                        >
                            Login
                        </Button>
                        <div className="flex justify-center">
                            <FormDescription>
                                Don't have an account?{" "}
                                <Link
                                    href="/auth/register"
                                    className="text-blue-500"
                                >
                                    Sign up
                                </Link>
                            </FormDescription>
                        </div>
                    </form>
                </Form>
            </div>
        </div>
    );
}
export default LoginForm;

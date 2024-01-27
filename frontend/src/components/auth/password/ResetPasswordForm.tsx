"use client";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { APP_NAME } from "@/lib/constants";
import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";
import { useMutation } from "react-query";
import { useToast } from "@/components/ui/use-toast";
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { PostResetPassword } from "@/lib/api/auth/resetPassword";

const formSchema = z.object({
    password: z.string().min(2),
    confirmPassword: z.string().min(2),
});

export type ConfirmPasswordFormType = z.infer<typeof formSchema>;
function ResetPasswordForm() {
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            confirmPassword: "",
            password: "",
        },
    });
    const router = useRouter();
    const { toast } = useToast();
    const searchParams = useSearchParams();
    const token = searchParams.get("token");

    //TODO: Maybe precheck if token is valid

    const handleSubmitResetPassword = useMutation(PostResetPassword, {
        onSuccess: (data) => {
            toast({
                title: "Success",
                description: data.message,
            });
            router.push("/");
        },
        onError: (error: any) => {
            toast({
                title: "Error",
                description: error.error,
            });
        },
    });

    function onsubmit(e: z.infer<typeof formSchema>) {
        if (token) {
            handleSubmitResetPassword.mutate({ ...e, token });
        } else {
            toast({
                title: "Error",
                description: "Token is missing",
            });
        }
    }

    return (
        <div className="w-full col-span-2  px-8 h-full flex items-center justify-center flex-col gap-8 max-w-xl">
            <h2 className="text-4xl text-pretty font-bold ">
                Reset Password for
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
                            name="password"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Password</FormLabel>
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
                        <FormField
                            control={form.control}
                            name="confirmPassword"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Password</FormLabel>
                                    <FormControl>
                                        <Input
                                            id="confirm-password"
                                            type="password"
                                            placeholder="Confirm Password"
                                            {...field}
                                        />
                                    </FormControl>

                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        <Button
                            // disabled={handleSubmitLogin.isLoading}
                            className={`w-full disabled:bg-gray-500 disabled:cursor-not-allowed disabled:dark:bg-gray-800`}
                            type="submit"
                        >
                            Reset Password
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
export default ResetPasswordForm;

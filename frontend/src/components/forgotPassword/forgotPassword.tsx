"use client";
import { zodResolver } from "@hookform/resolvers/zod";
import { useState } from "react";
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
} from "../ui/form";
import { Input } from "../ui/input";
import { Button } from "../ui/button";
import { APP_NAME } from "@/lib/constants";
import Link from "next/link";

import { useToast } from "../ui/use-toast";
import { forgotPasswordEmailRequest } from "@/lib/api/auth/forgotPassword";
import { useMutation } from "react-query";

const formSchema = z.object({
    email: z.string().email(),
});

export type ForgotPasswordFormData = z.infer<typeof formSchema>;

function ForgetPasswordForm() {
    const [loading, setLoading] = useState(false);
    const [message, setMessage] = useState("");
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            email: "",
        },
    });
    const { toast } = useToast();
    const handleForgotPassword = useMutation(forgotPasswordEmailRequest, {
        onSuccess: (data) => {
            toast({
                title: "Success",
                description: data.message,
            });
            setMessage(data.message);
        },
        onError: (error: any) => {
            toast({
                title: "Error",
                description: error.error,
            });
        },
    });

    function onsubmit(e: z.infer<typeof formSchema>) {
        handleForgotPassword.mutate(e);
    }
    return (
        <div className="w-full col-span-2  px-8 h-full flex items-center justify-center flex-col gap-8 max-w-xl">
            <h2 className="text-4xl text-pretty font-bold ">
                Forgot Password for
                <span className="text-blue-500"> {APP_NAME} </span> Account
            </h2>
            <h3 className="text-start w-full text-lg">
                Don't worry We Got you👋{" "}
            </h3>
            <div className="w-full flex flex-col">
                {message && <p className="bg-green-400">{message}</p>}
                <Form {...form}>
                    <form
                        onSubmit={form.handleSubmit(onsubmit)}
                        className="space-y-4 bg-gray-200 p-8 rounded-md dark:bg-gray-800"
                    >
                        <h3 className="text-center text-xl">Login</h3>
                        <FormField
                            control={form.control}
                            name="email"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Email</FormLabel>
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

                        <div className="flex justify-end">
                            <Link href="/auth/login" className="text-blue-500">
                                Go Back to Sign-in Page?
                            </Link>
                        </div>
                        <Button
                            disabled={loading}
                            className={`w-full disabled:bg-gray-500 disabled:cursor-not-allowed disabled:dark:bg-gray-800`}
                            type="submit"
                        >
                            Request Password Reset
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
export default ForgetPasswordForm;

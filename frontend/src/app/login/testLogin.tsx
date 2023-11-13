"use client";

import axios from "axios";
import { useState } from "react";
import { CurrentUserData } from "../layout";
import { useDispatch } from "@/lib/redux/store";
import { login } from "@/lib/redux/slices/auth";

type Props = {};
function TestLogin({}: Props) {
  const [email, setEmail] = useState("");
  const dispatch = useDispatch();
  const [password, setPassword] = useState("");
  async function onsubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const res = await axios.post(
      "http://localhost:5000/users/login/",
      {
        email,
        password,
      },
      { withCredentials: true }
    );
    const data = res.data as CurrentUserData;
    if (!data) return;
    dispatch(login({ ...data, isAuthenticated: true }));
  }
  return (
    <div>
      <form onSubmit={onsubmit}>
        <label htmlFor="username">Username:</label>
        <input
          onChange={(e) => {
            setEmail(e.currentTarget.value);
          }}
          value={email}
          type="text"
          id="username"
          name="username"
        />
        <label htmlFor="password">Password:</label>
        <input
          onChange={(e) => {
            setPassword(e.currentTarget.value);
          }}
          value={password}
          type="text"
          id="password"
          name="password"
        />
        <button>Submit</button>
      </form>
    </div>
  );
}
export default TestLogin;

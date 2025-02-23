import axios from "axios";
import { useRouter } from "next/navigation";
import React, { useEffect, useState, useContext } from "react";
import toast from "react-hot-toast";

const UserContext = React.createContext();

// set axios to include credentials with every request
axios.defaults.withCredentials = true;

export const UserContextProvider = ({ children }) => {
  const server_url = "";

  const router = useRouter();
  const [user, setUser] = useState({});
  const [loading, setLoading] = useState(false);
  const [allUsers, setAllUsers] = useState([]);
  const [userState, setUserState] = useState({
    name: "",
    email: "",
    password: "",
  });

  const [loggedIn, setLoggedIn] = useState(false);

  // register user
  const registerUser = async (e) => {
    setLoggedIn(false);
    e.preventDefault();
    if (
      !userState.email.includes("@") ||
      !userState.password ||
      userState.password.length < 6
    ) {
      toast.error("Please enter a valid email and password (min 6 characters)");
      return;
    }

    try {
      const res = await axios.post(
        `${server_url}/api/v1/auth/register`,
        userState
      );
      console.log("User registered successfully", res.data);
      setLoggedIn(true);
      toast.success("User registered successfully");

      // clear the form
      setUserState({
        name: "",
        email: "",
        password: "",
      });

      const loginUserData = response.data;
      // ローカルストレージにトークンを保存
      window.localStorage.setItem("login_token", loginUserData.token);

      // redirect to login page
      router.push(`${server_url}/login`);
    } catch (error) {
      console.log("Error registering user", error);
      toast.error(error.response.data.message);
    }
  };

  // login the user
  const loginUser = async (e) => {
    setLoggedIn(false);
    e.preventDefault();
    try {
      const res = await axios.post(
        `${server_url}/api/v1/auth/login`,
        {
          email: userState.email,
          password: userState.password,
        },
        {
          withCredentials: true, // send cookies to the server
        }
      );

      setLoggedIn(true);
      toast.success("User logged in successfully");

      // clear the form
      setUserState({
        email: "",
        password: "",
      });

      // refresh the user details
      await getUser(); // fetch before redirecting

      // push user to the dashboard page
      router.push("/");
    } catch (error) {
      console.log("Error logging in user", error);
      toast.error(error.response.data.message);
    }
  };

  // get user Looged in Status
  const userLoginStatus = async () => {
    setLoggedIn(false);
    try {
      const token = window.localStorage.getItem("login_token");
      if (token === "" || token === "undefined") {
        console.log(token);
        return false;
      }

      const res = await axios.get(`${server_url}/api/v1/auth/login-status`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
        withCredentials: true, // send cookies to the server
      });

      const currentUserData = response.data;
      currentUserData.token = token;

      return currentUserData;

      // // coerce the string to boolean
      // if (!!res.data) {
      //   setLoggedIn(true);
      // }

      // setLoading(false);

      // if (!loggedIn) {
      //   router.push("/login");
      // } else {
      //   await getUser();
      // }
    } catch (error) {
      console.log("Error getting user login status", error);
      return false;
    }

    // return loggedIn;
  };

  // logout user
  const logoutUser = async () => {
    try {
      const res = await axios.get(`${server_url}/api/v1/auth/logout`, {
        withCredentials: true, // send cookies to the server
      });

      toast.success("User logged out successfully");

      setUser({});

      // redirect to login page
      router.push("/login");
    } catch (error) {
      console.log("Error logging out user", error);
      toast.error(error.response.data.message);
    }
  };

  // get user details
  const getUser = async () => {
    setLoading(true);
    try {
      const res = await axios.get(`${server_url}/api/v1/auth/user`, {
        withCredentials: true, // send cookies to the server
      });

      setUser((prevState) => {
        return {
          ...prevState,
          ...res.data,
        };
      });

      setLoading(false);
    } catch (error) {
      console.log("Error getting user details", error);
      setLoading(false);
      toast.error(error.response.data.message);
    }
  };

  // update user details
  const updateUser = async (e, data) => {
    e.preventDefault();
    setLoading(true);

    try {
      const res = await axios.patch(`${server_url}/api/v1/auth/user`, data, {
        withCredentials: true, // send cookies to the server
      });

      // update the user state
      setUser((prevState) => {
        return {
          ...prevState,
          ...res.data,
        };
      });

      toast.success("User updated successfully");

      setLoading(false);
    } catch (error) {
      console.log("Error updating user details", error);
      setLoading(false);
      toast.error(error.response.data.message);
    }
  };

  // email verification
  const emailVerification = async () => {
    setLoading(true);
    try {
      const res = await axios.post(
        `${server_url}/api/v1/auth/verify-email`,
        {},
        {
          withCredentials: true, // send cookies to the server
        }
      );

      toast.success("Email verification sent successfully");
      setLoading(false);
    } catch (error) {
      console.log("Error sending email verification", error);
      setLoading(false);
      toast.error(error.response.data.message);
    }
  };

  // verify user/email
  const verifyUser = async (token) => {
    setLoading(true);
    try {
      const res = await axios.post(
        `${server_url}/api/v1/auth/verify-user/${token}`,
        {},
        {
          withCredentials: true, // send cookies to the server
        }
      );

      toast.success("User verified successfully");

      // refresh the user details
      getUser();

      setLoading(false);
      // redirect to home page
      router.push("/");
    } catch (error) {
      console.log("Error verifying user", error);
      toast.error(error.response.data.message);
      setLoading(false);
    }
  };

  // forgot password email
  const forgotPasswordEmail = async (email) => {
    setLoading(true);

    try {
      const res = await axios.post(
        `${server_url}/api/v1/auth/forgot-password`,
        {
          email,
        },
        {
          withCredentials: true, // send cookies to the server
        }
      );

      toast.success("Forgot password email sent successfully");
      setLoading(false);
    } catch (error) {
      console.log("Error sending forgot password email", error);
      toast.error(error.response.data.message);
      setLoading(false);
    }
  };

  // change password
  const changePassword = async (currentPassword, newPassword) => {
    setLoading(true);

    try {
      const res = await axios.patch(
        `${server_url}/api/v1/change-password`,
        { currentPassword, newPassword },
        {
          withCredentials: true, // send cookies to the server
        }
      );

      toast.success("Password changed successfully");
      setLoading(false);
    } catch (error) {
      console.log("Error changing password", error);
      toast.error(error.response.data.message);
      setLoading(false);
    }
  };

  // dynamic form handler
  const handlerUserInput = (name) => (e) => {
    const value = e.target.value;

    setUserState((prevState) => ({
      ...prevState,
      [name]: value,
    }));
  };

  // healthchecker
  const healthcheck = async (e) => {
    try {
      const res = await axios.get(`${server_url}/api/v1/auth/healthcheck`);
      console.log("API is ", res.data.message);
      toast.success(`API is ${res.data.message}`);
    } catch (error) {
      console.log("API is inactive", error);
      toast.error(error.response.data.message);
    }
  };

  useEffect(() => {
    const loginStatusGetUser = async () => {
      try {
        const user = await userLoginStatus();

        if (user) {
          setUser(user);
        } else {
          console.log("ユーザーが見つかりませんでした");
          setUser(null);
        }
      } catch (error) {
        console.error("ユーザーの取得に失敗しました");
        setUser(null);
      } finally {
        setLoading(false);
      }
    };

    loginStatusGetUser();
  }, [userLoginStatus, getUser]);

  return (
    <UserContext.Provider
      value={{
        registerUser,
        userState,
        handlerUserInput,
        loginUser,
        logoutUser,
        userLoginStatus,
        user,
        updateUser,
        emailVerification,
        verifyUser,
        forgotPasswordEmail,
        changePassword,
        healthcheck,
        loggedIn,
      }}
    >
      {children}
    </UserContext.Provider>
  );
};

export const useUserContext = () => {
  return useContext(UserContext);
};

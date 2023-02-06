import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import axios from "axios";
import Router, { PrivateRoute } from "./Routes";
import { Link, Route, useNavigate } from "react-router-dom";
import { useRecoilState, useRecoilValue } from "recoil";
import { authAtom } from "./atoms/auth";

const App = () => {
  return (
    <Router>
      <Route path="/" element={<App0 />} />
      <Route
        path="/mypage"
        element={
          <PrivateRoute>
            <MyPage />
          </PrivateRoute>
        }
      />
    </Router>
  );
};

const MyPage = () => {
  const auth = useRecoilValue(authAtom);
  return (
    <div>
      <h1>My Page</h1>
      <div>{auth.name}</div>
      <Link to="/">Home</Link>
    </div>
  );
};

function App0() {
  const [count, setCount] = useState(0);
  const [name, setName] = useState("");
  const [password, setPassword] = useState("");
  const [auth, setAuth] = useRecoilState(authAtom);
  const navigate = useNavigate();

  const Login = () => {
    axios
      .post(
        "http://localhost:8080/login",
        {
          name: name,
        },
        { withCredentials: true }
      )
      .then((res) => {
        console.log(res);
        if (res.status === 200) {
          console.log("Login success");
          const data = res.data;
          setAuth({ ...auth, isLogin: true, name: data.name });
        } else {
          const error = new Error(res.statusText);
          throw error;
        }
      });
  };

  const SignUp = () => {
    axios
      .post(
        "http://localhost:8080/users/add",
        {
          name: name,
        },
        { withCredentials: true }
      )
      .then((res) => {
        console.log(res);
        if (res.status === 200) {
          console.log("Sign up successfully");
          const data = res.data;
          console.log(data);
        } else {
          const error = new Error(res.statusText);
          throw error;
        }
      });
  };
  const GetInfo = () => {
    axios
      .get("http://localhost:8080/mypage", { withCredentials: true })
      .then((res) => {
        console.log(res);
        if (res.status === 200) {
          console.log(res.data);
        } else {
          const error = new Error(res.statusText);
          throw error;
        }
      });
  };

  return (
    <div className="App">
      <div>
        <div>
          <Link to="/">Home</Link>
        </div>
        <div>
          <Link to="/mypage">My Page</Link>
        </div>
        <div>{auth.name}</div>
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo" alt="Vite logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
      <div>
        <input
          type="text"
          name="name"
          onChange={(e) => setName(e.target.value)}
        />
        <input
          type="password"
          name="password"
          onChange={(e) => setPassword(e.target.value)}
        />
        <button type="submit" onClick={SignUp}>
          Sing Up
        </button>
      </div>
      {/* <form> */}
      <div>
        <input
          type="text"
          name="name"
          onChange={(e) => setName(e.target.value)}
        />
        <input
          type="password"
          name="password"
          onChange={(e) => setPassword(e.target.value)}
        />
        <button type="submit" onClick={Login}>
          Login
        </button>
      </div>
      {/* </form> */}
      <button onClick={GetInfo}>Get Info</button>
    </div>
  );
}

export default App;

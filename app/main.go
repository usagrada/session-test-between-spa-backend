package main

import (
	"fmt"
	"html/template"
	"io"
	"net/http"
	"todo-server/db"

	"github.com/gorilla/sessions"
	"github.com/labstack/echo-contrib/session"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"golang.org/x/oauth2"
)

type Template struct {
	templates *template.Template
}

func (t *Template) Render(w io.Writer, name string, data interface{}, c echo.Context) error {
	return t.templates.ExecuteTemplate(w, name, data)
}

type User struct {
	Name     string `json:"name" xml:"name" form:"name" query:"name"`
	Password string `json:"password" xml:"password" form:"password" query:"password"`
}

func main() {
	fmt.Println("Hello World")
	db.SetupDB()

	t := &Template{
		templates: template.Must(template.ParseGlob("views/*.html")),
	}

	e := echo.New()
	e.Renderer = t

	// Middleware
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	e.Use(session.Middleware(sessions.NewCookieStore([]byte("secret"))))

	// Routes
	e.GET("/hello", hello)
	e.GET("/users", get_users)
	e.GET("/users/add/:name", add_user)
	e.GET("/", func(c echo.Context) error {
		// テンプレートに渡す値

		data := struct {
			Content_a string
			Content_b string
			Content_c string
			Content_d string
		}{
			Content_a: "雨が降っています。",
			Content_b: "明日も雨でしょうか。",
			Content_c: "台風が近づいています。",
			Content_d: "Jun/11/2018",
		}
		return c.Render(http.StatusOK, "index", data)
	})
	e.GET("/login", func(c echo.Context) error {
		// テンプレートに渡す値
		return c.Render(http.StatusOK, "login", nil)
	})
	e.GET("/mypage", func(c echo.Context) error {
		session, _ := session.Get("session", c)
		if session.Values["loginCompleted"] == "completed" {
			username := session.Values["username"].(string)
			println(username)
			return c.String(http.StatusOK, "ログイン済みです")
		} else {
			return c.String(http.StatusOK, "ログインしてください")
		}
	})
	e.POST("/login", func(c echo.Context) error {
		user := new(User)
		if err := c.Bind(user); err != nil {
			return err
		}
		name := user.Name
		pass := user.Password
		println(name, pass)
		sess, _ := session.Get("session", c)
		sess.Options = &sessions.Options{
			Path:     "/",
			MaxAge:   86400 * 7,
			HttpOnly: true,
		}
		sess.Values["username"] = name
		sess.Values["loginCompleted"] = "completed"
		sess.Save(c.Request(), c.Response())
		// テンプレートに渡す値
		return c.Render(http.StatusOK, "login-ok", user)
	})

	e.Start(":8080")
}

func hello(c echo.Context) error {
	return c.String(http.StatusOK, "Hello, World!")
}

func get_users(c echo.Context) error {
	users := db.GetUsers()
	return c.JSON(http.StatusOK, users)
}

func add_user(c echo.Context) error {
	name := c.Param("name")
	println(name)
	db.AddUser(name)
	return c.JSON(http.StatusOK, "Added user {name}")
}

func sign_up(c echo.Context) error {
	oauth2Config := &oauth2.Config{
		ClientID: "client_id",
	}
	oauth2Config.AuthCodeURL("state")
	return c.String(http.StatusOK, "Hello, World!")
}

func log_in(c echo.Context) error {
	return c.String(http.StatusOK, "Hello, World!")
}

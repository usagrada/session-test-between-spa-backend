package db

import (
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

type Product struct {
	gorm.Model
	Code  string
	Price uint
}

type User struct {
	gorm.Model
	Name  string
	Email string
}

var conn *gorm.DB

func SetupDB() {
	db, err := gorm.Open(sqlite.Open("gorm.db"), &gorm.Config{})
	if err != nil {
		panic("failed to connect database")
	}
	db.AutoMigrate(&Product{})
	db.AutoMigrate(&User{})

	conn = db
}

func GetUsers() []User {
	var users []User
	conn.Find(&users)
	return users
}

func AddUser(name string) User {
	user := User{Name: name, Email: "test@email.com"}
	conn.Create(&user)
	return user
}

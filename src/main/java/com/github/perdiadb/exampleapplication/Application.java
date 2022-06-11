package com.github.perdiadb.exampleapplication;

import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.stage.Stage;

import java.io.IOException;

import at.davideko.perdia.queries.Template;

public class Application extends javafx.application.Application {
    @Override
    public void start(Stage stage) throws IOException {
        //Template t = new Template("Balls");
        FXMLLoader fxmlLoader = new FXMLLoader(Application.class.getResource("index.fxml"));
        Scene scene = new Scene(fxmlLoader.load(), 800, 600);
        stage.setTitle("Example Application");
        stage.setScene(scene);
        stage.show();
    }

    public static void main(String[] args) {
        launch();
    }
}
module com.github.perdiadb.exampleapplication {
    requires javafx.controls;
    requires javafx.fxml;

    requires org.controlsfx.controls;

    opens com.github.perdiadb.exampleapplication to javafx.fxml;
    exports com.github.perdiadb.exampleapplication;
}
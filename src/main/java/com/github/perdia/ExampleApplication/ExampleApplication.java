package com.github.perdia.ExampleApplication;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import com.github.perdia.Core;
import com.github.perdia.tcp.TCPClient;

@SpringBootApplication
@RestController
public class ExampleApplication {

	public static TCPClient client;

	public static void main(String[] args) {
		client = Core.init();
		SpringApplication.run(ExampleApplication.class, args);
	}

}

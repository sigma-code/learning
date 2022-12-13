package com.example.demo.ctrl;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class DemoCtrl {
  
  @GetMapping("/")
  public String helloWorld() {
    return "{\"msg\":\"Hello World\"}";
  }
}


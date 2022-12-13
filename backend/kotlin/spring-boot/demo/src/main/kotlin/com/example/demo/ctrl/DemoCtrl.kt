package com.example.demo.ctrl

import org.springframework.web.bind.annotation.RestController
import org.springframework.web.bind.annotation.GetMapping

@RestController
class DemoCtrl {
  
  @GetMapping("/")
  fun helloWorld(): String {
    return "{\"msg\":\"Hello World\"}"
  }
}


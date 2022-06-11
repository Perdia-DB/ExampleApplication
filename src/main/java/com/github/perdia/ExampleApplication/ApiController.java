package com.github.perdia.ExampleApplication;
import java.util.ArrayList;
import java.util.stream.Collectors;

import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.ResponseStatus;

import com.github.perdia.ExampleApplication.Blog.BlogInner;
import com.github.perdia.ExampleApplication.Blog.BlogObject;

import com.github.perdia.queries.Instance;
import com.github.perdia.queries.storage.AllInstances;

@Controller
public class ApiController {
    
    @GetMapping(path="/blogs")
    public @ResponseBody Iterable<BlogInner> getAllBlogs() {
        AllInstances.fetch(ExampleApplication.client);
        ArrayList<Instance> everything = AllInstances.get();

        ArrayList<BlogInner> blogs = (ArrayList<BlogInner>) everything.stream()
            .filter(entry -> entry.getTemplate() == BlogObject.self)
            .map(entry -> new BlogObject(entry))
            .map(entry -> entry.inner)
            .collect(Collectors.toList());

        return blogs;
    }

    @GetMapping(path="/blog")
    public @ResponseBody BlogInner getBlog(@RequestParam(value="title") String title) {
        ArrayList<BlogInner> blogs = (ArrayList<BlogInner>) getAllBlogs();

        BlogInner blog = blogs.stream()
            .filter(entry -> entry.title.equals(title))
            .collect(Collectors.toList()).get(0);

        return blog;
    }

    @GetMapping(path="/mock")
    public @ResponseBody BlogInner getMock() {
        return new BlogInner("Title", "Author", "Content", 5);
    }

    @PostMapping(path="/add-blog")
    public ResponseEntity<String> addBlog(@RequestBody BlogInner inner) {
        new BlogObject(inner);
        return new ResponseEntity<>("Successfully created Blog", HttpStatus.CREATED);
    }

}

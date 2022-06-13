package com.github.perdia.ExampleApplication;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.DeleteMapping;
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
@RequestMapping(path="/api")
public class ApiController {
    
    public Iterable<BlogInner> getAllBlogs() {
        AllInstances.fetch(ExampleApplication.client);
        ArrayList<Instance> everything = AllInstances.get();

        List<BlogInner> blogs = everything.stream()
            .filter(entry -> entry.getTemplate().getName().equals(BlogObject.self.getName()))
            .map(entry -> new BlogObject(entry).inner)
            .collect(Collectors.toList());

        return blogs;
    }

    @GetMapping(path="/blog")
    public @ResponseBody Iterable<BlogInner> getBlog(@RequestParam(required = false) String title) {
        if (title == null) {
            return getAllBlogs();
        }

        List<BlogInner> blog = ((List<BlogInner>) getAllBlogs()).stream()
            .filter(entry -> entry.title.equals(title))
            .collect(Collectors.toList());

        return blog;
    }

    @PostMapping(path="/blog")
    public ResponseEntity<String> addBlog(@RequestBody(required = true) BlogInner inner) {
        new BlogObject(inner).create();
        return new ResponseEntity<>("Successfully created Blog", HttpStatus.CREATED);
    }

    @DeleteMapping(path="/blog")
    public ResponseEntity<String> deleteBlog(@RequestParam(required = true) String title) {
        AllInstances.fetch(ExampleApplication.client);
        ArrayList<Instance> everything = AllInstances.get();

        List<BlogObject> blogs = everything.stream()
            .filter(entry -> entry.getTemplate().getName().equals(BlogObject.self.getName()))
            .map(entry -> new BlogObject(entry))
            .filter(entry -> entry.inner.title.equals(title))
            .collect(Collectors.toList());
            
        if (blogs.size() == 0) {
            return new ResponseEntity<>("No Blog with that title was found!", HttpStatus.NOT_FOUND);
        }
        Instance blog = blogs.get(0).create();
        ExampleApplication.client.write(blog.deleteQuery().getBytes(StandardCharsets.UTF_8));
        return new ResponseEntity<>("Successfully deleted Blog", HttpStatus.OK);
    }
}

package com.github.perdia.ExampleApplication.Blog;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

@JsonIgnoreProperties(ignoreUnknown = true)
public class BlogInner {
    public String title;
    public String author;
    public String content;
    public int likes;

    public BlogInner(String title, String author, String content, int likes) {
        this.title = title;
        this.author = author;
        this.content = content;
        this.likes = likes;
    }

    @Override
    public String toString() {
        return String.format("Blog: {%n   title=%s,%n   author=%s,%n   content=%s,%n   likes=%d%n}", this.title, this.author, this.content, this.likes);
    }
}

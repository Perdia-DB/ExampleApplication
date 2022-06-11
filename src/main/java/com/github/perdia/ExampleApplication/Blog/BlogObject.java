package com.github.perdia.ExampleApplication.Blog;

import java.nio.charset.StandardCharsets;
import java.util.HashMap;

import com.github.perdia.ExampleApplication.ExampleApplication;

import com.github.perdia.queries.Instance;
import com.github.perdia.queries.Template;
import com.github.perdia.queries.data.DataEntry;
import com.github.perdia.queries.data.DataType;
import com.github.perdia.queries.data.LongDataEntry;
import com.github.perdia.queries.data.StringDataEntry;
import com.github.perdia.queries.storage.AllInstances;
import com.github.perdia.queries.storage.AllTemplates;

public class BlogObject {
    public BlogInner inner;
    public static Template self;

    public static void init() {
        Template blog = new Template("BLOG");
        blog.addEntry("title", DataType.STRING, "");
        blog.addEntry("author", DataType.STRING, "");
        blog.addEntry("content", DataType.STRING, "");
        blog.addEntry("likes", DataType.INTEGER, 0);
        BlogObject.self = blog;
    }

    public BlogObject(Instance instance) {
        if (BlogObject.self == null) {
            BlogObject.init();
        }
        HashMap<String, DataEntry> data = instance.getData();
        String title = (String) data.get("title").read();
        String author = (String) data.get("author").read();
        String content = (String) data.get("content").read();
        int likes = (Integer) data.get("likes").read();
        this.inner = new BlogInner(title, author, content, likes);
        this.create();
    }

    public BlogObject(BlogInner inner) {
        if (BlogObject.self == null) {
            BlogObject.init();
        }
        this.inner = inner;
        this.create();
    }

    public BlogObject(String title, String author, String content) {
        this(title, author, content, 0);
    }

    public BlogObject(String title, String author, String content, int likes) {
        if (BlogObject.self == null) {
            BlogObject.init();
        }
        this.inner = new BlogInner(title, author, content, likes);
        this.create();
    }

    public void create() {
        Instance blog = new Instance("Blog", BlogObject.self);
        blog.setData("title", new StringDataEntry(this.inner.title));
        blog.setData("author", new StringDataEntry(this.inner.author));
        blog.setData("content", new StringDataEntry(this.inner.content));
        blog.setData("likes", new LongDataEntry((long) this.inner.likes));
        ExampleApplication.client.write(blog.createInstance(BlogObject.self).getBytes(StandardCharsets.UTF_8));
    }
}

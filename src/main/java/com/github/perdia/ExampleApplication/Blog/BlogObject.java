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
    public Instance inst;

    public static void init() {
        Template blog = new Template("BLOG");
        blog.addEntry("title", DataType.STRING, "");
        blog.addEntry("author", DataType.STRING, "");
        blog.addEntry("content", DataType.STRING, "");
        blog.addEntry("likes", DataType.INTEGER, 0);
        ExampleApplication.client.write(blog.toCreationQuery().getBytes(StandardCharsets.UTF_8));
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
    }

    public BlogObject(BlogInner inner) {
        if (BlogObject.self == null) {
            BlogObject.init();
        }
        this.inner = inner;
    }

    public BlogObject(String title, String author, String content) {
        this(title, author, content, 0);
    }

    public BlogObject(String title, String author, String content, int likes) {
        if (BlogObject.self == null) {
            BlogObject.init();
        }
        this.inner = new BlogInner(title, author, content, likes);
    }

    public Instance create() {
        if (this.inst != null) {
            return this.inst;
        }
        Instance blog = new Instance(this.inner.title, BlogObject.self);
        ExampleApplication.client.write(blog.allInOneQuery().getBytes(StandardCharsets.UTF_8));
        this.inst = blog;
        return this.inst;
    }
}

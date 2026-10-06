// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//
// Android library project for the plugin. Mirrors the upstream Tauri plugin
// scaffolded `android/build.gradle.kts` (see
// crates/tauri-cli/templates/plugin/android/build.gradle.kts).
//
// The Android package ID defaults to `com.plugin.<name_snake_case>`. When you
// copy this template, rename `com.plugin.template` everywhere (the namespace,
// the Kotlin folder path, and the `register_android_plugin("...")` call in
// src/mobile.rs).

import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.plugin.template"
    compileSdk = 37

    defaultConfig {
        minSdk = 21
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
}

kotlin {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_1_8
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.9.0")
    implementation("androidx.appcompat:appcompat:1.6.0")
    implementation("com.google.android.material:material:1.7.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    implementation(project(":tauri-android"))
}

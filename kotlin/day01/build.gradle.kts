plugins {
    kotlin("jvm") version "1.6.0"
}

group = "org.codeanon.aoc"
version = "0.1"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    implementation(project(":platform"))
}
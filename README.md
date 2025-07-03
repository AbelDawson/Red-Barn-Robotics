# 🌾 Crop Tracking System — Agricultural Robotics Take-Home Challenge

## 📋 Overview

This project implements a simplified crop tracking system that maintains **persistent object IDs** across video frames. It is designed to simulate the needs of agricultural robots performing precision operations in the field, where accurate and consistent object tracking is critical.

The solution runs as a **Dockerized command-line tool**, processes input detections from a JSON file, and outputs both tracked data and visualizations.

---

## 🚜 Problem Context

Agricultural robots often encounter dynamic environments — objects may temporarily disappear due to occlusions or camera movement. This system tracks crops across multiple frames and assigns **consistent IDs** to reappearing objects.

---

## ✅ Features

- 🔁 Persistent object tracking using object position and motion
- 🧠 Handles temporary disappearance of objects (up to 1–3 frames)
- 🖼️ Visualizes tracked objects and their history
- 📦 Fully containerized (Docker) with a single entrypoint
- 📝 MIT licensed for open contribution and reuse

---

## 🧪 Example Usage

### Docker Run

```bash
docker run --rm -v $(pwd):/data tracking-solution \
    --input test_data/input_data.json \
    --output tracking_output.json \
    --vis-dir visualization


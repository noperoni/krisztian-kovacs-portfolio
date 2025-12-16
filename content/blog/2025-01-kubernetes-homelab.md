---
slug: kubernetes-homelab-journey
date: 2025-01-10
tags: [kubernetes, homelab, devops, cloud]
title_en: My Kubernetes Homelab Journey
title_fr: Mon Parcours Kubernetes en Homelab
summary_en: From a single Raspberry Pi to a multi-node K3s cluster - lessons learned running Kubernetes at home.
summary_fr: D'un seul Raspberry Pi a un cluster K3s multi-noeud - les lecons apprises en executant Kubernetes a la maison.
---

<!-- EN -->
# My Kubernetes Homelab Journey

Running Kubernetes at home might seem like overkill, but it's the best way to truly understand container orchestration. Here's my journey from Docker Compose to a full K3s cluster.

## The Setup

My homelab consists of:

- **3x Intel NUCs** - The control plane and worker nodes
- **1x NAS** - For persistent storage via NFS
- **Unifi network** - VLANs for isolation

```yaml
# K3s server installation
apiVersion: v1
kind: ConfigMap
metadata:
  name: k3s-config
data:
  config.yaml: |
    cluster-init: true
    disable:
      - traefik
    tls-san:
      - k8s.local
```

## Lessons Learned

### 1. Start Simple

Don't try to replicate production immediately. Start with a single node, understand the basics, then expand.

### 2. Storage is Hard

Kubernetes storage can be tricky. I started with hostPath volumes, moved to NFS, and eventually settled on Longhorn for distributed storage.

### 3. GitOps is Essential

Managing manifests manually doesn't scale. ArgoCD changed everything - now all my configs live in Git.

## What's Running

Currently hosting:

- This portfolio website
- Plane (project management)
- Monitoring stack (Prometheus + Grafana)
- Various hobby projects

<!-- FR -->
# Mon Parcours Kubernetes en Homelab

Faire tourner Kubernetes a la maison peut sembler excessif, mais c'est la meilleure facon de vraiment comprendre l'orchestration de conteneurs. Voici mon parcours de Docker Compose a un cluster K3s complet.

## La Configuration

Mon homelab se compose de :

- **3x Intel NUCs** - Le control plane et les noeuds workers
- **1x NAS** - Pour le stockage persistant via NFS
- **Reseau Unifi** - VLANs pour l'isolation

```yaml
# Installation serveur K3s
apiVersion: v1
kind: ConfigMap
metadata:
  name: k3s-config
data:
  config.yaml: |
    cluster-init: true
    disable:
      - traefik
    tls-san:
      - k8s.local
```

## Lecons Apprises

### 1. Commencer Simple

N'essayez pas de repliquer la production immediatement. Commencez avec un seul noeud, comprenez les bases, puis etendez.

### 2. Le Stockage est Difficile

Le stockage Kubernetes peut etre delicat. J'ai commence avec des volumes hostPath, suis passe au NFS, et j'ai finalement opte pour Longhorn pour le stockage distribue.

### 3. GitOps est Essentiel

Gerer les manifests manuellement ne passe pas a l'echelle. ArgoCD a tout change - maintenant toutes mes configs vivent dans Git.

## Ce Qui Tourne

Actuellement heberge :

- Ce site portfolio
- Plane (gestion de projet)
- Stack de monitoring (Prometheus + Grafana)
- Divers projets personnels

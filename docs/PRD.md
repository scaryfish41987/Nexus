---
titulo: Nexus - Motor Visualizador de Algoritmos
version: 1.1
autor: Jaime Mendivil Martinez
fecha: 2026-03-18
estado: Obsoleto (Cambio a plataforma)
---
# PRD: Nexus - Motor Visualizador de Algoritmos (v1.1)

## 1. Visión y Propósito (El "Por qué")
* **El Problema:** Descargar y compartir archivos `.rhai` manualmente cada vez que el profesor corrige o añade un algoritmo genera versiones desactualizadas y fricción en los estudiantes.
* **La Solución:** Nexus evoluciona a una plataforma de contenido dinámico. Sincroniza silenciosamente una biblioteca oficial de algoritmos al abrirse, manteniendo la libertad de que el usuario modifique copias locales.

## 2. Público Objetivo (User Personas)
* **El Estudiante Principal:** Necesita garantías de que está estudiando con el algoritmo correcto proporcionado por la clase, pero requiere un entorno seguro ("sandbox") para experimentar rompiendo el código.
* **El Docente de Algoritmia:** Quiere actualizar un algoritmo en GitHub y saber que todos sus alumnos tendrán la versión visualizable en sus máquinas al día siguiente.

## 3. Casos de Uso (User Journeys)
* "Como estudiante, al abrir Nexus, quiero que la app descargue los nuevos algoritmos automáticamente en la carpeta `official/` sin que yo tenga que configurar nada."
* "Como estudiante curioso, quiero copiar un algoritmo a mi carpeta `custom/`, alterar la lógica del *while*, y que Nexus ejecute mi versión para ver cómo se corrompe el ordenamiento."

## 4. Alcance del MVP (In Scope vs. Out of Scope)
* **In Scope (En Alcance):**
    * Todo lo de la v1.0.
    * `SyncManager`: Sincronización automática con repositorio remoto (GitHub API).
    * Sistema de carpetas separadas (`official/` vs `custom/`).
* **Out of Scope (Fuera de Alcance):**
    * Grafos 2D/3D.
    * Actualización remota del ejecutable principal (OTA Update).

## 5. Requisitos a Alto Nivel
* **Funcionales:** Detección y priorización de scripts locales sobre los oficiales si hay conflicto de nombres; indicadores visuales en la UI (ej. etiqueta "Oficial" o "Personalizado").
* **No Funcionales / Experiencia:** La comprobación de red al arranque no debe bloquear la apertura de la interfaz (debe ser asíncrona).

## 6. Métricas de Éxito (KPIs)
* **Cuantitativas:** El peso de la red al sincronizar debe ser < 500 KB; el arranque completo (incluyendo ping de red) debe mantenerse < 1.5s.
* **Cualitativas:** El estudiante entiende intuitivamente que no debe editar los archivos de la carpeta oficial gracias a las advertencias de la interfaz.

## 7. Dependencias y Riesgos
* **Técnicos:** Límites de peticiones (Rate Limits) de la API de GitHub si muchos estudiantes abren la aplicación al mismo tiempo.
* **De Recursos:** (Mismos que v1.0).

## 8. Cronograma / Hitos (Milestones)
* **Hito 1:** Implementación del `SyncManager` y lectura de múltiples directorios.
* **Hito 2:** Integración de la lógica de sincronización con la UI de Dioxus.
* **Hito 3:** MVP listo con catálogo de algoritmos auto-actualizable.


# Documentación del Proyecto: Gestión de Cohetes en Agencia Espacial

## Descripción general
Este programa en Solana implementa un sistema CRUD (Crear, Leer, Actualizar, Eliminar) para gestionar cohetes dentro de una bodega espacial. El objetivo es mantener un registro controlado de los cohetes disponibles, permitiendo al propietario de la bodega administrar:

- La creación de una bodega (espacio de almacenamiento on-chain)
- Agregar nuevos cohetes a la bodega
- Consultar la lista de cohetes
- Actualizar el estado de disponibilidad de cada cohete
- Eliminar cohetes de la bodega

El control de acceso está basado en la wallet del propietario; solo él puede realizar modificaciones en la bodega y sus cohetes.

## Estructura principal

### 1. Bodega
Representa la cuenta principal del programa que almacena la información de la bodega.

Se define con un struct que contiene:
- **owner:** Pubkey — Dirección pública del dueño.
- **nombre:** String — Nombre de la bodega.
- **cohetes:** Vec<Cohete> — Vector con los cohetes gestionados (hasta 10 elementos máximo).

### 2. Cohete
Representa cada cohete dentro de la bodega.

Se define con campos:
- **nombre:** String — Nombre del cohete.
- **modelo:** String — Modelo del cohete.
- **disponible:** bool — Estado que indica si el cohete está disponible o no.

## Contextos para las instrucciones
El programa define contextos (structs) que describen las cuentas y permisos necesarios para ejecutar las instrucciones:

- **NuevaBodega**  
  Contexto para crear una nueva bodega.  
  Requiere que un owner (wallet signer) pague y cree una cuenta bodega que es una PDA derivada de su PublicKey y un seed específico (`b"bodega"`).  
  Incluye el programa del sistema para la creación de cuenta.

- **NuevoCohete**  
  Contexto para agregar, eliminar, consultar o actualizar cohetes.  
  Requiere que el owner sea quien firme la transacción.  
  La cuenta bodega debe ser mutable porque se modificarán sus datos (cohetes).

## Funciones CRUD

### Crear Bodega
- **Función:** `crear_bodega`  
- Crea una bodega vacía asignada al owner.  
- Inicializa el vector de cohetes vacío.  
- Recibe el nombre de la bodega.  
- Se crea la cuenta tipo PDA para almacenar la información.

### Agregar Cohete
- **Función:** `agregar_cohete`  
- Añade un cohete nuevo a la bodega.  
- Se deben enviar nombre y modelo.  
- El cohete se marca inicialmente como disponible (`disponible = true`).  
- Solo el dueño de la bodega puede agregar.

### Consultar Cohetes
- **Función:** `ver_cohete`  
- Lista todos los cohetes actualmente almacenados en la bodega.  
- Solo el dueño puede consultarlo.  
- Muestra un log con la información detallada.

### Actualizar Estado de Cohete
- **Función:** `alternar_estado`  
- Cambia el estado `disponible` de un cohete específico (`true/false`).  
- Se busca el cohete por nombre y se alterna su disponibilidad.  
- Solo el dueño puede modificar.

### Eliminar Cohete
- **Función:** `eliminar_cohete`  
- Elimina un cohete de la bodega si existe.  
- Se busca por nombre para eliminar.  
- Solo el propietario puede realizar esta operación.

## Manejo de errores
El programa define errores explícitos para mejorar la experiencia y seguridad:
- **NoEresElOwner** — Cuando un usuario que no es dueño intenta modificar la bodega o los cohetes.
- **CoheteNoExiste** — Cuando se intenta modificar o eliminar un cohete que no está en la bodega.

## Consideraciones técnicas
- Se usan PDAs para la cuenta bodega para asegurar propiedad única y eliminación de claves privadas.
- El espacio asignado para la cuenta se calcula con `Bodega::INIT_SPACE + 8` bytes.
- Se limita la cantidad máxima de cohetes a 10 para controlar el tamaño de la cuenta.
- El estado y datos de los cohetes se guardan on-chain como parte del struct `Bodega`.
- Requiere que la wallet del usuario firme cualquier transacción que modifique estados.

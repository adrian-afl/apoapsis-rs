import { Vector2, Vector3, Vector4 } from "@aeroflightlabs/linear-math";

export interface GeometryMemoryLayout {
  attributes: {
    format:
      | "int8"
      | "uint8"
      | "int16"
      | "uint16"
      | "int32"
      | "uint32"
      | "float16"
      | "float32"
      | "padding8"
      | "padding16"
      | "padding32";
    normalize: boolean; // causes int values to be cast to -1 to 1 or 0 to 1 floats in shaders
    dimensions: 1 | 2 | 3 | 4;
  }[];
}

export class Object3dIntermediateVertex {
  public position: Vector3;
  public uv: Vector2;
  public normal: Vector3;
  public tangent: Vector4;

  public constructor(
    position: Vector3,
    uv: Vector2,
    normal: Vector3,
    tangent: Vector4 = new Vector4(),
  ) {
    this.position = position;
    this.uv = uv;
    this.normal = normal;
    this.tangent = tangent;
  }
}

export class Object3dIntermediate {
  public readonly vertices: Object3dIntermediateVertex[];
  public constructor(vertices: Object3dIntermediateVertex[]) {
    this.vertices = vertices;
    this.recalculateTangents();
  }

  public getVertexArray(
    components: ("position" | "normal" | "uv" | "tangent")[],
  ): {
    data: ArrayBuffer;
    layout: GeometryMemoryLayout;
  } {
    const verticesCount = this.vertices.length;

    const layoutAttributes: GeometryMemoryLayout["attributes"] = [];
    let vertexFloatSize = 0;

    for (const component of components) {
      if (component === "position") {
        layoutAttributes.push({
          dimensions: 3,
          normalize: false,
          format: "float32",
        });
        vertexFloatSize += 3;
      }
      if (component === "normal") {
        layoutAttributes.push({
          dimensions: 3,
          normalize: false,
          format: "float32",
        });
        vertexFloatSize += 3;
      }
      if (component === "uv") {
        layoutAttributes.push({
          dimensions: 2,
          normalize: false,
          format: "float32",
        });
        vertexFloatSize += 2;
      }
      if (component === "tangent") {
        layoutAttributes.push({
          dimensions: 4,
          normalize: false,
          format: "float32",
        });
        vertexFloatSize += 4;
      }
    }

    const arr = new Float32Array(verticesCount * vertexFloatSize);
    let g = 0;
    for (let i = 0; i < verticesCount; i++) {
      const v = this.vertices[i];
      for (const component of components) {
        if (component === "position") {
          arr[g++] = v.position.x;
          arr[g++] = v.position.y;
          arr[g++] = v.position.z;
        }
        if (component === "normal") {
          arr[g++] = v.normal.x;
          arr[g++] = v.normal.y;
          arr[g++] = v.normal.z;
        }
        if (component === "uv") {
          arr[g++] = v.uv.x;
          arr[g++] = v.uv.y;
        }
        if (component === "tangent") {
          arr[g++] = v.tangent.x;
          arr[g++] = v.tangent.y;
          arr[g++] = v.tangent.z;
          arr[g++] = v.tangent.w;
        }
      }
    }
    return {
      data: arr.buffer,
      layout: {
        attributes: layoutAttributes,
      },
    };
  }

  public getTriangles(): Vector3[][] {
    const triangles: Vector3[][] = [];
    const verticesCount = this.vertices.length;
    for (let i = 0; i < verticesCount; i += 3) {
      triangles.push([
        this.vertices[i].position,
        this.vertices[i + 1].position,
        this.vertices[i + 2].position,
      ]);
    }
    return triangles;
  }

  // vengine port, from 2015, i have forgotten how it works
  public recalculateTangents(): void {
    const t1a: Vector3[] = new Array<Vector3>();
    const t2a: Vector3[] = new Array<Vector3>();
    for (let i = 0; i < this.vertices.length; i += 3) {
      const vboIndex1 = i;
      const vboIndex2 = i + 1;
      const vboIndex3 = i + 2;

      const pos1 = this.vertices[vboIndex1].position;
      const pos2 = this.vertices[vboIndex2].position;
      const pos3 = this.vertices[vboIndex3].position;
      const uv1 = this.vertices[vboIndex1].uv;
      const uv2 = this.vertices[vboIndex2].uv;
      const uv3 = this.vertices[vboIndex3].uv;
      // const nor1 = this.vertices[vboIndex1].normal;
      // const nor2 = this.vertices[vboIndex2].normal;
      // const nor3 = this.vertices[vboIndex3].normal;

      const x1 = pos2.x - pos1.x;
      const x2 = pos3.x - pos1.x;

      const y1 = pos2.y - pos1.y;
      const y2 = pos3.y - pos1.y;

      const z1 = pos2.z - pos1.z;
      const z2 = pos3.z - pos1.z;

      const s1 = uv2.x - uv1.x;
      const s2 = uv3.x - uv1.x;

      const t1 = uv2.y - uv1.y;
      const t2 = uv3.y - uv1.y;

      const r = 1.0 / (s1 * t2 - s2 * t1);
      const sdir: Vector3 = new Vector3(
        (t2 * x1 - t1 * x2) * r,
        (t2 * y1 - t1 * y2) * r,
        (t2 * z1 - t1 * z2) * r,
      );
      const tdir: Vector3 = new Vector3(
        (s1 * x2 - s2 * x1) * r,
        (s1 * y2 - s2 * y1) * r,
        (s1 * z2 - s2 * z1) * r,
      );
      t1a.push(sdir);
      t1a.push(sdir);
      t1a.push(sdir);
      t2a.push(tdir);
      t2a.push(tdir);
      t2a.push(tdir);

      // const addedToTangents = new vec4([sdir.x, sdir.y, sdir.z, 0.0]);
      // this.vertices[vboIndex1].tangent = Vector.add(
      //   this.vertices[vboIndex1].tangent,
      //   addedToTangents
      // );
      // this.vertices[vboIndex2].tangent = Vector.add(
      //   this.vertices[vboIndex1].tangent,
      //   addedToTangents
      // );
      // this.vertices[vboIndex3].tangent = Vector.add(
      //   this.vertices[vboIndex1].tangent,
      //   addedToTangents
      // );
    }

    for (let i = 0; i < this.vertices.length; i++) {
      const vboIndex1 = i;

      const nor1 = this.vertices[vboIndex1].normal;
      const tan = t1a[i].clone();
      const dot = nor1.dot(tan);
      const nor1mulled = nor1.clone().scale(dot);
      const h = nor1.clone().cross(tan).dot(t2a[i]) < 0.0 ? -1.0 : 1.0;
      tan.subtract(nor1mulled).normalize().negate();

      this.vertices[vboIndex1].tangent = new Vector4(tan.x, tan.y, tan.z, h);
    }
  }

  public recalculateNormalsFlat(): void {
    const verticesCount = this.vertices.length;
    for (let i = 0; i < verticesCount; i += 3) {
      const v1 = this.vertices[i];
      const v2 = this.vertices[i + 1];
      const v3 = this.vertices[i + 2];
      v1.normal = v2.position
        .subtract(v1.position)
        .cross(v3.position.subtract(v1.position));
      v1.normal.normalize();
      v2.normal = v1.normal;
      v3.normal = v1.normal;
    }
  }
}

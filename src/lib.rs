#![deny(clippy::all)]

use napi::{bindgen_prelude::*, JsBuffer};
use napi_derive::napi;

pub struct GetAttributeTask {
  path: String,
  name: String,
}

#[napi]
impl Task for GetAttributeTask {
  type Output = Option<Vec<u8>>;
  type JsValue = Option<Buffer>;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(xattr::get(&self.path, &self.name).ok().flatten())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.map(|value| value.into()))
  }
}

#[napi]
pub fn get_attribute(path: String, name: String) -> AsyncTask<GetAttributeTask> {
  AsyncTask::new(GetAttributeTask { path, name })
}

#[napi]
pub fn get_attribute_sync(path: String, name: String) -> Option<Buffer> {
  xattr::get(path, name)
    .ok()
    .flatten()
    .map(|value| value.into())
}

pub struct SetAttributeTask {
  path: String,
  name: String,
  value: Either<Buffer, String>,
}

#[napi]
impl Task for SetAttributeTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    match &self.value {
      Either::A(buffer) => {
        xattr::set(&self.path, &self.name, buffer)?;
      }
      Either::B(s) => {
        xattr::set(&self.path, &self.name, s.as_bytes())?;
      }
    }

    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

#[napi]
pub fn set_attribute(
  path: String,
  name: String,
  value: Either<Buffer, String>,
) -> AsyncTask<SetAttributeTask> {
  AsyncTask::new(SetAttributeTask { path, name, value })
}

#[napi]
pub fn set_attribute_sync(
  path: String,
  name: String,
  value: Either<JsBuffer, String>,
) -> Result<()> {
  match value {
    Either::A(buffer) => {
      let buffer = buffer.into_value()?;
      xattr::set(path, name, buffer.as_ref())?;
    }
    Either::B(s) => {
      xattr::set(path, name, s.as_bytes())?;
    }
  };

  Ok(())
}

#[napi]
pub fn remove_attribute_sync(path: String, name: String) -> Result<()> {
  xattr::remove(path, name)?;
  Ok(())
}
pub struct RemoveAttributeTask {
  path: String,
  name: String,
}

#[napi]
impl Task for RemoveAttributeTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    xattr::remove(&self.path, &self.name)?;
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

#[napi]
pub fn remove_attribute(path: String, name: String) -> AsyncTask<RemoveAttributeTask> {
  AsyncTask::new(RemoveAttributeTask { path, name })
}

pub struct ListAttributesTask {
  path: String,
}

#[napi]
impl Task for ListAttributesTask {
  type Output = Vec<String>;
  type JsValue = Vec<String>;

  fn compute(&mut self) -> Result<Self::Output> {
    xattr::list(&self.path)?
      .map(|x| {
        x.into_string()
          .map_err(|err| Error::new(Status::InvalidArg, format!("Invalid attribute: {:?}", err)))
      })
      .collect()
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn list_attributes(path: String) -> AsyncTask<ListAttributesTask> {
  AsyncTask::new(ListAttributesTask { path })
}

#[napi]
pub fn list_attributes_sync(path: String) -> Result<Vec<String>> {
  xattr::list(path)?
    .map(|x| {
      x.into_string()
        .map_err(|err| Error::new(Status::InvalidArg, format!("Invalid attribute: {:?}", err)))
    })
    .collect()
}

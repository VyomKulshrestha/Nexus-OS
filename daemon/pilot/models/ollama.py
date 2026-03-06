"""Ollama HTTP API client."""

from __future__ import annotations

import logging

import httpx

logger = logging.getLogger("pilot.models.ollama")

DEFAULT_TIMEOUT = 600.0  # 10 minutes — local LLMs can be slow for complex plans


class OllamaModelNotFoundError(RuntimeError):
    def __init__(self, model: str, available: list[str]) -> None:
        self.model = model
        self.available = available
        avail_str = ", ".join(available) if available else "none"
        super().__init__(
            f"Model '{model}' is not installed in Ollama. "
            f"Available models: {avail_str}. "
            f"Run 'ollama pull {model}' to install it, or change the model in Settings."
        )


class OllamaClient:
    """Client for the Ollama local inference server."""

    def __init__(self, base_url: str = "http://127.0.0.1:11434") -> None:
        self._base_url = base_url.rstrip("/")
        self._client = httpx.AsyncClient(timeout=DEFAULT_TIMEOUT)

    async def is_available(self) -> bool:
        try:
            resp = await self._client.get(f"{self._base_url}/api/tags")
            return resp.status_code == 200
        except (httpx.ConnectError, httpx.TimeoutException):
            return False

    async def list_models(self) -> list[str]:
        try:
            resp = await self._client.get(f"{self._base_url}/api/tags")
            resp.raise_for_status()
            data = resp.json()
            return [m["name"] for m in data.get("models", [])]
        except (httpx.ConnectError, httpx.TimeoutException):
            return []

    async def generate(
        self,
        model: str,
        prompt: str,
        *,
        system: str = "",
        json_mode: bool = False,
        temperature: float = 0.1,
        stream: bool = False,
    ) -> str:
        """Generate a completion. Returns the full response text."""
        payload: dict = {
            "model": model,
            "prompt": prompt,
            "stream": stream,
            "options": {"temperature": temperature},
        }
        if system:
            payload["system"] = system
        if json_mode:
            payload["format"] = "json"

        resp = await self._client.post(
            f"{self._base_url}/api/generate",
            json=payload,
        )

        if resp.status_code == 404:
            available = await self.list_models()
            raise OllamaModelNotFoundError(model, available)

        resp.raise_for_status()
        data = resp.json()
        return data.get("response", "")

    async def chat(
        self,
        model: str,
        messages: list[dict[str, str]],
        *,
        json_mode: bool = False,
        temperature: float = 0.1,
    ) -> str:
        payload: dict = {
            "model": model,
            "messages": messages,
            "stream": False,
            "options": {"temperature": temperature},
        }
        if json_mode:
            payload["format"] = "json"

        resp = await self._client.post(
            f"{self._base_url}/api/chat",
            json=payload,
        )
        resp.raise_for_status()
        data = resp.json()
        return data.get("message", {}).get("content", "")

    async def close(self) -> None:
        await self._client.aclose()
